#[derive(Debug, Clone, PartialEq)]
pub enum StatusValue {
    None,
    Text(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
    List(Vec<StatusValue>),
}

impl From<&str> for StatusValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl From<String> for StatusValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<i64> for StatusValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for StatusValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for StatusValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayerStatusValue {
    Value(StatusValue),
    Multiscale {
        level: StatusValue,
        value: Option<StatusValue>,
    },
}

pub fn format_float(value: f64) -> String {
    if value.is_nan() {
        return "nan".to_owned();
    }
    if value.is_infinite() {
        return if value.is_sign_negative() {
            "-inf".to_owned()
        } else {
            "inf".to_owned()
        };
    }
    if value == 0.0 {
        return "0".to_owned();
    }

    let sign = if value.is_sign_negative() { "-" } else { "" };
    let absolute = value.abs();
    let precision = 3_i32;
    let exponent = absolute.log10().floor() as i32;

    if exponent < -4 || exponent >= precision {
        format_scientific(sign, absolute, exponent, precision)
    } else {
        format_fixed(sign, absolute, exponent, precision)
    }
}

pub fn status_format(value: &StatusValue) -> String {
    match value {
        StatusValue::None => String::new(),
        StatusValue::Text(value) => value.clone(),
        StatusValue::Integer(value) => value.to_string(),
        StatusValue::Float(value) => format_float(*value),
        StatusValue::Bool(value) => value.to_string(),
        StatusValue::List(values) => {
            let values = values.iter().map(status_format).collect::<Vec<_>>();
            format!("[{}]", values.join(", "))
        }
    }
}

pub fn generate_layer_status_strings(
    position: Option<&[f64]>,
    value: Option<&LayerStatusValue>,
) -> (String, String) {
    let position = position.map(format_position).unwrap_or_default();
    let value = value.map(format_layer_value).unwrap_or_default();
    (position, value)
}

pub fn generate_layer_coords_status(
    position: Option<&[f64]>,
    value: Option<&LayerStatusValue>,
) -> String {
    let (position, value) = generate_layer_status_strings(position, value);
    format!("{position}: {value}")
}

pub fn generate_layer_status(
    name: &str,
    position: Option<&[f64]>,
    value: Option<&LayerStatusValue>,
) -> String {
    let mut message = if let Some(position) = position {
        format!("{name}{}", format_position(position))
    } else {
        name.to_owned()
    };

    if let Some(value) = value {
        let value = format_layer_value(value);
        if !value.is_empty() {
            message.push_str(": ");
            message.push_str(&value);
        }
    }

    message
}

fn format_scientific(sign: &str, absolute: f64, mut exponent: i32, precision: i32) -> String {
    let mut mantissa = absolute / 10_f64.powi(exponent);
    let decimals = (precision - 1) as usize;
    let rounded = format!("{mantissa:.decimals$}");
    if rounded.starts_with("10") {
        exponent += 1;
        mantissa /= 10.0;
    }

    let mantissa = trim_float_string(format!("{mantissa:.decimals$}"));
    let exponent_sign = if exponent >= 0 { '+' } else { '-' };
    let exponent_abs = exponent.abs();
    format!("{sign}{mantissa}e{exponent_sign}{exponent_abs:02}")
}

fn format_fixed(sign: &str, absolute: f64, exponent: i32, precision: i32) -> String {
    let decimals = (precision - exponent - 1).max(0) as usize;
    let value = trim_float_string(format!("{absolute:.decimals$}"));
    format!("{sign}{value}")
}

fn trim_float_string(mut value: String) -> String {
    if value.contains('.') {
        while value.ends_with('0') {
            value.pop();
        }
        if value.ends_with('.') {
            value.pop();
        }
    }
    value
}

fn format_position(position: &[f64]) -> String {
    let rounded = position
        .iter()
        .map(|value| round_half_to_even(*value).to_string())
        .collect::<Vec<_>>()
        .join(" ");
    format!(" [{rounded}]")
}

fn round_half_to_even(value: f64) -> i64 {
    let floor = value.floor();
    let fraction = value - floor;
    if fraction < 0.5 {
        floor as i64
    } else if fraction > 0.5 {
        floor as i64 + 1
    } else {
        let floor_integer = floor as i64;
        if floor_integer % 2 == 0 {
            floor_integer
        } else {
            floor_integer + 1
        }
    }
}

fn format_layer_value(value: &LayerStatusValue) -> String {
    match value {
        LayerStatusValue::Value(value) => status_format(value),
        LayerStatusValue::Multiscale { level, value } => {
            if matches!(level, StatusValue::None) && value.as_ref() == Some(&StatusValue::None) {
                return String::new();
            }

            let mut value_string = status_format(level);
            if let Some(value) = value
                && !matches!(value, StatusValue::None)
            {
                value_string.push_str(", ");
                value_string.push_str(&status_format(value));
            }
            value_string
        }
    }
}
