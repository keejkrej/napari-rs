use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarycentricError {
    TriangleVertexCount(usize),
    DimensionMismatch,
    DegenerateTriangle,
}

impl fmt::Display for BarycentricError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TriangleVertexCount(count) => {
                write!(formatter, "triangle must have three vertices, got {count}")
            }
            Self::DimensionMismatch => {
                formatter.write_str("point and triangle vertices must have matching dimensions")
            }
            Self::DegenerateTriangle => formatter.write_str("triangle is degenerate"),
        }
    }
}

impl std::error::Error for BarycentricError {}

pub fn calculate_barycentric_coordinates(
    point: &[f64],
    triangle_vertices: &[Vec<f64>],
) -> Result<[f64; 3], BarycentricError> {
    if triangle_vertices.len() != 3 {
        return Err(BarycentricError::TriangleVertexCount(
            triangle_vertices.len(),
        ));
    }
    if triangle_vertices
        .iter()
        .any(|vertex| vertex.len() != point.len())
    {
        return Err(BarycentricError::DimensionMismatch);
    }

    let vertex_a = &triangle_vertices[0];
    let vertex_b = &triangle_vertices[1];
    let vertex_c = &triangle_vertices[2];
    let v0 = subtract(vertex_b, vertex_a);
    let v1 = subtract(vertex_c, vertex_a);
    let v2 = subtract(point, vertex_a);
    let d00 = dot(&v0, &v0);
    let d01 = dot(&v0, &v1);
    let d11 = dot(&v1, &v1);
    let d20 = dot(&v2, &v0);
    let d21 = dot(&v2, &v1);
    let denominator = d00 * d11 - d01 * d01;
    if denominator == 0.0 {
        return Err(BarycentricError::DegenerateTriangle);
    }

    let v = (d11 * d20 - d01 * d21) / denominator;
    let w = (d00 * d21 - d01 * d20) / denominator;
    let u = 1.0 - v - w;
    Ok([u, v, w])
}

fn subtract(left: &[f64], right: &[f64]) -> Vec<f64> {
    left.iter()
        .zip(right.iter())
        .map(|(left, right)| left - right)
        .collect()
}

fn dot(left: &[f64], right: &[f64]) -> f64 {
    left.iter()
        .zip(right.iter())
        .map(|(left, right)| left * right)
        .sum()
}
