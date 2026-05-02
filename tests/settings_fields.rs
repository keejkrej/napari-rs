use napari_rs::settings::fields::Version;

#[test]
fn version_parse_accepts_semver_core_prerelease_and_build_like_python_field() {
    let version: Version = "1.2.3-alpha.1+build.5".parse().unwrap();
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert_eq!(version.prerelease.as_deref(), Some("alpha.1"));
    assert_eq!(version.build.as_deref(), Some("build.5"));
    assert_eq!(
        version.to_tuple(),
        (1, 2, 3, Some("alpha.1"), Some("build.5"))
    );
}

#[test]
fn version_parse_rejects_invalid_semver_strings_like_python_regex() {
    for value in [
        "",
        "1",
        "1.2",
        "01.2.3",
        "1.02.3",
        "1.2.03",
        "1.2.3-01",
        "1.2.3-",
        "1.2.3+",
        "1.2.3+bad!",
    ] {
        assert!(value.parse::<Version>().is_err(), "{value} should fail");
    }
}

#[test]
fn version_stringification_matches_python_field_behavior() {
    assert_eq!(Version::new(1, 2, 3).to_string(), "1.2.3");
    assert_eq!(
        Version::new(1, 2, 3).with_prerelease("alpha").to_string(),
        "1.2.3alpha"
    );
    assert_eq!(
        Version::new(1, 2, 3).with_build("build.1").to_string(),
        "1.2.3build.1"
    );
}

#[test]
fn version_ordering_uses_numeric_parts_only_like_python_field() {
    let release: Version = "1.2.3".parse().unwrap();
    let prerelease: Version = "1.2.3-alpha".parse().unwrap();
    let build: Version = "1.2.3+build".parse().unwrap();
    let newer: Version = "1.2.4-alpha".parse().unwrap();

    assert_eq!(release, prerelease);
    assert_eq!(release, build);
    assert!(newer > release);
    assert!(Version::new(2, 0, 0) > Version::new(1, 99, 99));
}
