#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringEncodingSpec {
    Direct { feature: String },
    Format { format: String },
    Manual { array: Vec<String>, default: String },
}

pub fn validate_string_encoding_from_str(value: &str) -> StringEncodingSpec {
    if is_format_string(value) {
        StringEncodingSpec::Format {
            format: value.to_owned(),
        }
    } else {
        StringEncodingSpec::Direct {
            feature: value.to_owned(),
        }
    }
}

pub fn validate_string_encoding_from_sequence(values: &[String]) -> StringEncodingSpec {
    StringEncodingSpec::Manual {
        array: values.to_vec(),
        default: String::new(),
    }
}

pub fn is_format_string(value: &str) -> bool {
    parse_format_fields(value).is_some_and(|fields| !fields.is_empty())
}

pub fn parse_format_fields(value: &str) -> Option<Vec<String>> {
    let mut chars = value.chars().peekable();
    let mut fields = Vec::new();

    while let Some(char_) = chars.next() {
        match char_ {
            '{' => {
                if matches!(chars.peek(), Some('{')) {
                    chars.next();
                    continue;
                }
                let mut field = String::new();
                let mut found_end = false;
                for inner in chars.by_ref() {
                    if inner == '}' {
                        found_end = true;
                        break;
                    }
                    if inner == '{' {
                        return None;
                    }
                    field.push(inner);
                }
                if !found_end {
                    return None;
                }
                let field_name = field
                    .split(['!', ':'])
                    .next()
                    .expect("split always yields at least one segment")
                    .trim();
                fields.push(field_name.to_owned());
            }
            '}' => {
                if matches!(chars.peek(), Some('}')) {
                    chars.next();
                } else {
                    return None;
                }
            }
            _ => {}
        }
    }

    Some(fields)
}
