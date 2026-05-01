use napari_rs::utils::validators::{
    ValidationError, pairwise, validate_increasing, validate_n_seq, validate_n_seq_by,
};

#[test]
fn validate_n_seq_accepts_matching_length() {
    assert_eq!(validate_n_seq(&[4, 5], 2), Ok(()));
}

#[test]
fn validate_n_seq_rejects_wrong_length() {
    assert_eq!(
        validate_n_seq(&[1, 2, 3], 2),
        Err(ValidationError::Length {
            expected: 2,
            actual: 3
        })
    );
}

#[test]
fn validate_n_seq_by_checks_each_item_with_predicate() {
    assert_eq!(
        validate_n_seq_by(&[4, 5], 2, "positive integer", |value| *value > 0),
        Ok(())
    );
    assert_eq!(
        validate_n_seq_by(&[1, -5], 2, "positive integer", |value| *value > 0),
        Err(ValidationError::Type {
            expected: "positive integer",
            index: 1
        })
    );
}

#[test]
fn pairwise_matches_python_case() {
    assert_eq!(pairwise(&[1, 2, 3]), vec![(1, 2), (2, 3)]);
}

#[test]
fn validate_increasing_accepts_strictly_increasing_values() {
    assert_eq!(validate_increasing(&[1, 2, 3]), Ok(()));
}

#[test]
fn validate_increasing_rejects_decreasing_or_constant_values() {
    assert_eq!(
        validate_increasing(&[3, 2, 1]),
        Err(ValidationError::NotIncreasing)
    );
    assert_eq!(
        validate_increasing(&[1, 1, 2]),
        Err(ValidationError::NotIncreasing)
    );
}
