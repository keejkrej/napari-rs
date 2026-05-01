use napari_rs::utils::naming::{SEP, START, inc_name_count, numbered_count_match};

#[test]
fn numbered_count_match_matches_python_base_bracket_cases() {
    assert_eq!(numbered_count_match("layer [12]"), "12");
    assert_eq!(numbered_count_match("layer [e]"), "");
    assert_eq!(numbered_count_match("layer 12]"), "");
    assert_eq!(numbered_count_match("layer [12"), "");
    assert_eq!(numbered_count_match("layer[12]"), "");
    assert_eq!(numbered_count_match("layer 12"), "");
    assert_eq!(numbered_count_match("layer12"), "");
    assert_eq!(numbered_count_match("layer"), "");
}

#[test]
fn numbered_count_match_uses_last_valid_bracketed_count() {
    assert_eq!(numbered_count_match("layer [3] [123]"), "123");
}

#[test]
fn numbered_count_match_accepts_first_bracket() {
    assert_eq!(numbered_count_match(" [42]"), "42");
    assert_eq!(numbered_count_match("[42]"), "42");
}

#[test]
fn inc_name_count_matches_python_cases() {
    assert_eq!(inc_name_count("layer [7]"), "layer [8]");
    assert_eq!(inc_name_count("layer"), format!("layer{SEP}[{START}]"));
    assert_eq!(inc_name_count("[41]"), "[42]");
}

#[test]
fn inc_name_count_appends_when_final_brackets_are_not_a_valid_count() {
    assert_eq!(inc_name_count("layer [e]"), "layer [e] [1]");
    assert_eq!(inc_name_count("layer[12]"), "layer[12] [1]");
}

#[test]
fn inc_name_count_preserves_python_empty_bracket_behavior() {
    assert_eq!(inc_name_count("layer []"), "layer [ [1]]");
}

#[test]
fn inc_name_count_drops_leading_zeroes_like_python_int_conversion() {
    assert_eq!(inc_name_count("foo [001]"), "foo [2]");
}
