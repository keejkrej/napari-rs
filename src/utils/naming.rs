pub const SEP: &str = " ";
pub const START: u64 = 1;

pub fn numbered_count_match(name: &str) -> &str {
    bracketed_count_range(name)
        .map(|range| &name[range])
        .unwrap_or("")
}

pub fn inc_name_count(name: &str) -> String {
    match bracketed_count_range(name) {
        Some(range) => {
            let count = &name[range.clone()];
            match count.parse::<u64>() {
                Ok(count) => {
                    let mut incremented = name.to_owned();
                    incremented.replace_range(range, &(count + 1).to_string());
                    incremented
                }
                Err(_) => {
                    let mut incremented = name.to_owned();
                    incremented.insert_str(range.start, &format!("{SEP}[{START}]"));
                    incremented
                }
            }
        }
        None => format!("{name}{SEP}[{START}]"),
    }
}

fn bracketed_count_range(name: &str) -> Option<std::ops::Range<usize>> {
    if !name.ends_with(']') {
        return None;
    }

    let bracket_index = name.rfind('[')?;
    let bracket_is_at_start = bracket_index == 0;
    let bracket_follows_whitespace = name[..bracket_index]
        .chars()
        .next_back()
        .is_some_and(char::is_whitespace);
    if !bracket_is_at_start && !bracket_follows_whitespace {
        return None;
    }

    let count_start = bracket_index + '['.len_utf8();
    let count_end = name.len() - ']'.len_utf8();
    let count = &name[count_start..count_end];
    if count.chars().all(|char| char.is_ascii_digit()) {
        Some(count_start..count_end)
    } else {
        None
    }
}
