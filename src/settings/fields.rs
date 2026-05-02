use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseVersionError {
    value: String,
}

impl fmt::Display for ParseVersionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} is not valid SemVer string", self.value)
    }
}

impl std::error::Error for ParseVersionError {}

#[derive(Clone, Debug, Eq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub prerelease: Option<String>,
    pub build: Option<String>,
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.numeric_tuple() == other.numeric_tuple()
    }
}

impl Version {
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            prerelease: None,
            build: None,
        }
    }

    pub fn with_prerelease(mut self, prerelease: impl Into<String>) -> Self {
        self.prerelease = Some(prerelease.into());
        self
    }

    pub fn with_build(mut self, build: impl Into<String>) -> Self {
        self.build = Some(build.into());
        self
    }

    pub const fn numeric_tuple(&self) -> (u64, u64, u64) {
        (self.major, self.minor, self.patch)
    }

    pub fn to_tuple(&self) -> (u64, u64, u64, Option<&str>, Option<&str>) {
        (
            self.major,
            self.minor,
            self.patch,
            self.prerelease.as_deref(),
            self.build.as_deref(),
        )
    }
}

impl fmt::Display for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(prerelease) = &self.prerelease {
            formatter.write_str(prerelease)?;
        }
        if let Some(build) = &self.build {
            formatter.write_str(build)?;
        }
        Ok(())
    }
}

impl FromStr for Version {
    type Err = ParseVersionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        parse_version(value)
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.numeric_tuple().cmp(&other.numeric_tuple())
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_version(value: &str) -> Result<Version, ParseVersionError> {
    let error = || ParseVersionError {
        value: value.to_string(),
    };
    let (core_and_pre, build) = split_once_optional(value, '+');
    if let Some(build) = build {
        validate_dot_identifiers(build, false).ok_or_else(error)?;
    }

    let (core, prerelease) = split_once_optional(core_and_pre, '-');
    if let Some(prerelease) = prerelease {
        validate_dot_identifiers(prerelease, true).ok_or_else(error)?;
    }

    let mut parts = core.split('.');
    let major =
        parse_numeric_identifier(parts.next().ok_or_else(error)?, false).ok_or_else(error)?;
    let minor =
        parse_numeric_identifier(parts.next().ok_or_else(error)?, false).ok_or_else(error)?;
    let patch =
        parse_numeric_identifier(parts.next().ok_or_else(error)?, false).ok_or_else(error)?;
    if parts.next().is_some() {
        return Err(error());
    }

    Ok(Version {
        major,
        minor,
        patch,
        prerelease: prerelease.map(str::to_string),
        build: build.map(str::to_string),
    })
}

fn split_once_optional(value: &str, delimiter: char) -> (&str, Option<&str>) {
    match value.split_once(delimiter) {
        Some((left, right)) => (left, Some(right)),
        None => (value, None),
    }
}

fn parse_numeric_identifier(value: &str, allow_leading_zero: bool) -> Option<u64> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    if !allow_leading_zero && value.len() > 1 && value.starts_with('0') {
        return None;
    }
    value.parse().ok()
}

fn validate_dot_identifiers(value: &str, reject_numeric_leading_zero: bool) -> Option<()> {
    if value.is_empty() {
        return None;
    }
    for part in value.split('.') {
        if part.is_empty()
            || !part
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        {
            return None;
        }
        if reject_numeric_leading_zero
            && part.bytes().all(|byte| byte.is_ascii_digit())
            && part.len() > 1
            && part.starts_with('0')
        {
            return None;
        }
    }
    Some(())
}
