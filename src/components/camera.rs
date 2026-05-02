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

    pub fn set_view_direction(
        &mut self,
        view_direction: [f64; 3],
        up_direction: Option<[f64; 3]>,
    ) -> Result<(), CameraError> {
        let up_direction = up_direction.unwrap_or([0.0, -1.0, 0.0]);
        let projection_scale = dot3(up_direction, view_direction);
        let projected_up = [
            up_direction[0] - projection_scale * view_direction[0],
            up_direction[1] - projection_scale * view_direction[1],
            up_direction[2] - projection_scale * view_direction[2],
        ];

        let view_direction = normalize3(view_direction).ok_or(CameraError::ZeroViewDirection)?;
        let up_direction = normalize3(projected_up).ok_or(CameraError::ParallelUpDirection)?;
        let right_direction = cross3(up_direction, view_direction);
        let matrix = [
            [-view_direction[0], -view_direction[1], -view_direction[2]],
            [-up_direction[0], -up_direction[1], -up_direction[2]],
            [
                -right_direction[0],
                -right_direction[1],
                -right_direction[2],
            ],
        ];
        self.angles = xyz_euler_degrees_from_rotation_matrix(matrix);
        Ok(())
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
pub enum CameraError {
    ZeroViewDirection,
    ParallelUpDirection,
}

impl fmt::Display for CameraError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroViewDirection => formatter.write_str("view direction cannot be zero"),
            Self::ParallelUpDirection => {
                formatter.write_str("up direction cannot be parallel to view direction")
            }
        }
    }
}

impl std::error::Error for CameraError {}

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

fn xyz_euler_degrees_from_rotation_matrix(matrix: [[f64; 3]; 3]) -> [f64; 3] {
    let sy = -matrix[2][0];
    let cy = (matrix[0][0] * matrix[0][0] + matrix[1][0] * matrix[1][0]).sqrt();
    let (rx, ry, rz) = if cy > 1e-12 {
        (
            matrix[2][1].atan2(matrix[2][2]),
            sy.atan2(cy),
            matrix[1][0].atan2(matrix[0][0]),
        )
    } else {
        ((-matrix[1][2]).atan2(matrix[1][1]), sy.atan2(cy), 0.0)
    };
    [rx.to_degrees(), ry.to_degrees(), rz.to_degrees()]
}

fn dot3(left: [f64; 3], right: [f64; 3]) -> f64 {
    left.iter()
        .zip(right)
        .map(|(left, right)| left * right)
        .sum()
}

fn cross3(left: [f64; 3], right: [f64; 3]) -> [f64; 3] {
    [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]
}

fn normalize3(vector: [f64; 3]) -> Option<[f64; 3]> {
    let norm = vector.iter().map(|value| value * value).sum::<f64>().sqrt();
    if norm <= 1e-12 {
        return None;
    }
    Some(vector.map(|value| value / norm))
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
