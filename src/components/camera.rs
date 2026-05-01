use std::fmt;
use std::str::FromStr;

use crate::utils::camera_orientations::{
    DEFAULT_ORIENTATION, DepthAxisOrientation, Handedness, HorizontalAxisOrientation,
    VerticalAxisOrientation,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub center: [f64; 3],
    pub zoom: f64,
    pub angles: [f64; 3],
    pub perspective: f64,
    pub mouse_pan: bool,
    pub mouse_zoom: bool,
    pub orientation: (
        DepthAxisOrientation,
        VerticalAxisOrientation,
        HorizontalAxisOrientation,
    ),
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            center: [0.0, 0.0, 0.0],
            zoom: 1.0,
            angles: [0.0, 0.0, 0.0],
            perspective: 0.0,
            mouse_pan: true,
            mouse_zoom: true,
            orientation: DEFAULT_ORIENTATION,
        }
    }
}

impl Camera {
    pub fn view_direction(&self) -> [f64; 3] {
        let rotation = rotation_matrix_from_xyz_euler_degrees(self.angles);
        [-rotation[0][0], -rotation[0][1], -rotation[0][2]]
    }

    pub fn up_direction(&self) -> [f64; 3] {
        let rotation = rotation_matrix_from_xyz_euler_degrees(self.angles);
        [-rotation[1][0], -rotation[1][1], -rotation[1][2]]
    }

    pub fn calculate_nd_view_direction(
        &self,
        ndim: usize,
        dims_displayed: &[usize],
    ) -> Option<Vec<f64>> {
        embed_3d_direction(ndim, dims_displayed, self.view_direction())
    }

    pub fn calculate_nd_up_direction(
        &self,
        ndim: usize,
        dims_displayed: &[usize],
    ) -> Option<Vec<f64>> {
        embed_3d_direction(ndim, dims_displayed, self.up_direction())
    }

    pub fn orientation2d(&self) -> (VerticalAxisOrientation, HorizontalAxisOrientation) {
        (self.orientation.1, self.orientation.2)
    }

    pub fn set_orientation2d(
        &mut self,
        vertical: VerticalAxisOrientation,
        horizontal: HorizontalAxisOrientation,
    ) {
        self.orientation = (self.orientation.0, vertical, horizontal);
    }

    pub fn handedness(&self) -> Handedness {
        let diffs = [
            self.orientation.0 != DEFAULT_ORIENTATION.0,
            self.orientation.1 != DEFAULT_ORIENTATION.1,
            self.orientation.2 != DEFAULT_ORIENTATION.2,
        ];
        if diffs.into_iter().filter(|differs| *differs).count() % 2 != 0 {
            Handedness::Left
        } else {
            Handedness::Right
        }
    }

    pub fn vispy_flipped_axes(&self, ndisplay: DisplayDimensions) -> [u8; 3] {
        let default_orientation = match ndisplay {
            DisplayDimensions::Two => (
                HorizontalAxisOrientation::Right,
                VerticalAxisOrientation::Up,
                DepthAxisOrientation::Towards,
            ),
            DisplayDimensions::Three => (
                HorizontalAxisOrientation::Right,
                VerticalAxisOrientation::Down,
                DepthAxisOrientation::Away,
            ),
        };
        let orientation_xyz = (self.orientation.2, self.orientation.1, self.orientation.0);

        [
            u8::from(orientation_xyz.0 != default_orientation.0),
            u8::from(orientation_xyz.1 != default_orientation.1),
            u8::from(orientation_xyz.2 != default_orientation.2),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayDimensions {
    Two,
    Three,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseDisplayDimensionsError;

impl fmt::Display for ParseDisplayDimensionsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "display dimensions must be 2 or 3")
    }
}

impl std::error::Error for ParseDisplayDimensionsError {}

impl TryFrom<usize> for DisplayDimensions {
    type Error = ParseDisplayDimensionsError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            _ => Err(ParseDisplayDimensionsError),
        }
    }
}

impl FromStr for DisplayDimensions {
    type Err = ParseDisplayDimensionsError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            _ => Err(ParseDisplayDimensionsError),
        }
    }
}

fn embed_3d_direction(
    ndim: usize,
    dims_displayed: &[usize],
    direction: [f64; 3],
) -> Option<Vec<f64>> {
    if dims_displayed.len() != 3 {
        return None;
    }

    let mut direction_nd = vec![0.0; ndim];
    for (&axis, value) in dims_displayed.iter().zip(direction) {
        direction_nd[axis] = value;
    }
    Some(direction_nd)
}

fn rotation_matrix_from_xyz_euler_degrees(angles: [f64; 3]) -> [[f64; 3]; 3] {
    let [rx, ry, rz] = angles.map(f64::to_radians);
    let (sin_x, cos_x) = rx.sin_cos();
    let (sin_y, cos_y) = ry.sin_cos();
    let (sin_z, cos_z) = rz.sin_cos();

    let rotation_x = [[1.0, 0.0, 0.0], [0.0, cos_x, -sin_x], [0.0, sin_x, cos_x]];
    let rotation_y = [[cos_y, 0.0, sin_y], [0.0, 1.0, 0.0], [-sin_y, 0.0, cos_y]];
    let rotation_z = [[cos_z, -sin_z, 0.0], [sin_z, cos_z, 0.0], [0.0, 0.0, 1.0]];

    mat3_mul(mat3_mul(rotation_z, rotation_y), rotation_x)
}

fn mat3_mul(left: [[f64; 3]; 3], right: [[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut result = [[0.0; 3]; 3];
    for row in 0..3 {
        for col in 0..3 {
            result[row][col] = (0..3)
                .map(|inner| left[row][inner] * right[inner][col])
                .sum();
        }
    }
    result
}
