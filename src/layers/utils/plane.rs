use std::fmt;

use crate::utils::geometry::{Vec3, intersect_line_with_plane_3d};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane {
    pub position: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(position: Vec3, normal: Vec3) -> Result<Self, PlaneError> {
        Ok(Self {
            position,
            normal: normalize3(normal)?,
        })
    }

    pub fn shift_along_normal_vector(&mut self, distance: f64) {
        for (position, normal) in self.position.iter_mut().zip(self.normal) {
            *position += distance * normal;
        }
    }

    pub fn intersect_with_line(&self, line_position: Vec3, line_direction: Vec3) -> Vec3 {
        intersect_line_with_plane_3d(line_position, line_direction, self.position, self.normal)
    }

    pub fn from_points(a: Vec3, b: Vec3, c: Vec3) -> Result<Self, PlaneError> {
        let ab = sub3(b, a);
        let ac = sub3(c, a);
        let normal = cross3(ab, ac);
        let position = [
            (a[0] + b[0] + c[0]) / 3.0,
            (a[1] + b[1] + c[1]) / 3.0,
            (a[2] + b[2] + c[2]) / 3.0,
        ];
        Self::new(position, normal)
    }

    pub fn as_array(&self) -> [[f64; 3]; 2] {
        [self.position, self.normal]
    }

    pub fn from_array(array: [[f64; 3]; 2]) -> Result<Self, PlaneError> {
        Self::new(array[0], array[1])
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            normal: [1.0, 0.0, 0.0],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SlicingPlane {
    pub plane: Plane,
    pub thickness: f64,
}

impl SlicingPlane {
    pub fn new(position: Vec3, normal: Vec3, thickness: f64) -> Result<Self, PlaneError> {
        Ok(Self {
            plane: Plane::new(position, normal)?,
            thickness,
        })
    }

    pub fn from_array(array: [[f64; 3]; 2], thickness: f64) -> Result<Self, PlaneError> {
        Ok(Self {
            plane: Plane::from_array(array)?,
            thickness,
        })
    }

    pub fn as_array(&self) -> [[f64; 3]; 2] {
        self.plane.as_array()
    }
}

impl Default for SlicingPlane {
    fn default() -> Self {
        Self {
            plane: Plane::default(),
            thickness: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClippingPlane {
    pub plane: Plane,
    pub enabled: bool,
}

impl ClippingPlane {
    pub fn new(position: Vec3, normal: Vec3, enabled: bool) -> Result<Self, PlaneError> {
        Ok(Self {
            plane: Plane::new(position, normal)?,
            enabled,
        })
    }

    pub fn from_array(array: [[f64; 3]; 2], enabled: bool) -> Result<Self, PlaneError> {
        Ok(Self {
            plane: Plane::from_array(array)?,
            enabled,
        })
    }

    pub fn as_array(&self) -> [[f64; 3]; 2] {
        self.plane.as_array()
    }
}

impl Default for ClippingPlane {
    fn default() -> Self {
        Self {
            plane: Plane::default(),
            enabled: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClippingPlaneList {
    planes: Vec<ClippingPlane>,
}

impl ClippingPlaneList {
    pub fn new(planes: impl Into<Vec<ClippingPlane>>) -> Self {
        Self {
            planes: planes.into(),
        }
    }

    pub fn as_array(&self) -> Vec<[[f64; 3]; 2]> {
        self.planes
            .iter()
            .filter(|plane| plane.enabled)
            .map(ClippingPlane::as_array)
            .collect()
    }

    pub fn from_array(
        array: impl Into<Vec<[[f64; 3]; 2]>>,
        enabled: bool,
    ) -> Result<Self, PlaneError> {
        let planes = array
            .into()
            .into_iter()
            .map(|array| ClippingPlane::from_array(array, enabled))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::new(planes))
    }

    pub fn from_bounding_box(
        center: Vec3,
        dimensions: Vec3,
        enabled: bool,
    ) -> Result<Self, PlaneError> {
        let mut planes = Vec::with_capacity(6);
        for axis in 0..3 {
            for direction in [-1.0, 1.0] {
                let mut position = center;
                position[axis] += (dimensions[axis] / 2.0) * direction;

                let mut normal = [0.0, 0.0, 0.0];
                normal[axis] = -direction;
                planes.push(ClippingPlane::new(position, normal, enabled)?);
            }
        }
        Ok(Self::new(planes))
    }

    pub fn add_plane(&mut self, plane: ClippingPlane) {
        self.planes.push(plane);
    }

    pub fn len(&self) -> usize {
        self.planes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.planes.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&ClippingPlane> {
        self.planes.get(index)
    }

    pub fn planes(&self) -> &[ClippingPlane] {
        &self.planes
    }
}

impl Default for ClippingPlaneList {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaneError {
    ZeroNormal,
}

impl fmt::Display for PlaneError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroNormal => formatter.write_str("plane normal vector cannot be zero"),
        }
    }
}

impl std::error::Error for PlaneError {}

fn normalize3(vector: Vec3) -> Result<Vec3, PlaneError> {
    let norm = (vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2]).sqrt();
    if norm == 0.0 {
        return Err(PlaneError::ZeroNormal);
    }
    Ok([vector[0] / norm, vector[1] / norm, vector[2] / norm])
}

fn sub3(left: Vec3, right: Vec3) -> Vec3 {
    [left[0] - right[0], left[1] - right[1], left[2] - right[2]]
}

fn cross3(left: Vec3, right: Vec3) -> Vec3 {
    [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]
}
