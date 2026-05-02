#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unit {
    Pixel,
    Meter,
    Centimeter,
    Millimeter,
    Micrometer,
    Nanometer,
}

impl Unit {
    pub fn base_scale(&self) -> f64 {
        match self {
            Self::Pixel => 1.0,
            Self::Meter => 1.0,
            Self::Centimeter => 0.01,
            Self::Millimeter => 0.001,
            Self::Micrometer => 0.000_001,
            Self::Nanometer => 0.000_000_001,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnitsError {
    UnknownUnit(String),
}

impl std::fmt::Display for UnitsError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownUnit(unit) => write!(formatter, "Could not find unit {unit}"),
        }
    }
}

impl std::error::Error for UnitsError {}

#[derive(Debug, Clone, PartialEq)]
pub struct Quantity {
    pub magnitude: f64,
    pub unit: Unit,
}

pub fn get_unit_from_name(unit: Option<&str>) -> Result<Unit, UnitsError> {
    let Some(unit) = unit else {
        return Ok(Unit::Pixel);
    };
    match unit {
        "pixel" | "pixels" | "px" => Ok(Unit::Pixel),
        "m" | "meter" | "meters" | "metre" | "metres" => Ok(Unit::Meter),
        "cm" | "centimeter" | "centimeters" | "centimetre" | "centimetres" => Ok(Unit::Centimeter),
        "mm" | "millimeter" | "millimeters" | "millimetre" | "millimetres" => Ok(Unit::Millimeter),
        "um" | "micrometer" | "micrometers" | "micrometre" | "micrometres" => Ok(Unit::Micrometer),
        "nm" | "nanometer" | "nanometers" | "nanometre" | "nanometres" => Ok(Unit::Nanometer),
        _ => Err(UnitsError::UnknownUnit(unit.to_owned())),
    }
}

pub fn get_units_from_names(units: Option<&[&str]>) -> Result<Vec<Unit>, UnitsError> {
    match units {
        None => Ok(vec![Unit::Pixel]),
        Some(units) => units
            .iter()
            .map(|unit| get_unit_from_name(Some(unit)))
            .collect(),
    }
}

pub fn pixel_units(ndim: usize) -> Vec<Unit> {
    vec![Unit::Pixel; ndim]
}

pub fn default_axis_labels(ndim: usize) -> Vec<String> {
    (1..=ndim).rev().map(|axis| format!("-{axis}")).collect()
}
