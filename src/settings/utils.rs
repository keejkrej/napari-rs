use std::collections::BTreeMap;

pub fn coerce_extensions_to_globs<T: Clone>(
    reader_settings: &BTreeMap<String, T>,
) -> BTreeMap<String, T> {
    reader_settings
        .iter()
        .map(|(pattern, reader)| {
            let pattern = if pattern.starts_with('.') && !pattern.contains('*') {
                format!("*{pattern}")
            } else {
                pattern.clone()
            };
            (pattern, reader.clone())
        })
        .collect()
}
