use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;
use std::time::SystemTime;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseNotificationSeverityError {
    value: String,
}

impl fmt::Display for ParseNotificationSeverityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown notification severity: {}", self.value)
    }
}

impl std::error::Error for ParseNotificationSeverityError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NotificationSeverity {
    None,
    Debug,
    Info,
    Warning,
    Error,
}

impl NotificationSeverity {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Debug => "debug",
            Self::None => "none",
        }
    }

    pub const fn level(self) -> u8 {
        match self {
            Self::Error => 40,
            Self::Warning => 30,
            Self::Info => 20,
            Self::Debug => 10,
            Self::None => 0,
        }
    }

    pub const fn as_icon(self) -> &'static str {
        match self {
            Self::Error => "ⓧ",
            Self::Warning => "⚠️",
            Self::Info => "ⓘ",
            Self::Debug => "🐛",
            Self::None => "",
        }
    }
}

impl fmt::Display for NotificationSeverity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for NotificationSeverity {
    type Err = ParseNotificationSeverityError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "error" => Ok(Self::Error),
            "warning" => Ok(Self::Warning),
            "info" => Ok(Self::Info),
            "debug" => Ok(Self::Debug),
            "none" => Ok(Self::None),
            _ => Err(ParseNotificationSeverityError {
                value: value.to_string(),
            }),
        }
    }
}

impl PartialOrd for NotificationSeverity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NotificationSeverity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.level().cmp(&other.level())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Notification {
    pub message: String,
    pub severity: NotificationSeverity,
    pub actions: Vec<String>,
    pub date: SystemTime,
}

impl Notification {
    pub fn new(message: impl Into<String>, severity: NotificationSeverity) -> Self {
        Self {
            message: message.into(),
            severity,
            actions: Vec::new(),
            date: SystemTime::now(),
        }
    }

    pub fn with_actions(
        message: impl Into<String>,
        severity: NotificationSeverity,
        actions: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            message: message.into(),
            severity,
            actions: actions.into_iter().map(Into::into).collect(),
            date: SystemTime::now(),
        }
    }
}

impl fmt::Display for Notification {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {}",
            self.severity.as_str().to_ascii_uppercase(),
            self.message
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WarningNotification {
    pub warning_type: String,
    pub message: String,
    pub filename: Option<String>,
    pub lineno: Option<usize>,
    pub notification: Notification,
}

impl WarningNotification {
    pub fn new(
        message: impl Into<String>,
        warning_type: impl Into<String>,
        filename: Option<impl Into<String>>,
        lineno: Option<usize>,
    ) -> Self {
        let message = message.into();
        Self {
            notification: Notification::new(message.clone(), NotificationSeverity::Warning),
            warning_type: warning_type.into(),
            message,
            filename: filename.map(Into::into),
            lineno,
        }
    }
}

impl fmt::Display for WarningNotification {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}: {}: {}!",
            self.filename.as_deref().unwrap_or("None"),
            self.lineno
                .map(|line| line.to_string())
                .unwrap_or_else(|| "None".to_string()),
            self.warning_type,
            self.message
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct WarningKey {
    message: String,
    category: String,
    filename: String,
    lineno: usize,
}

#[derive(Clone, Debug)]
pub struct NotificationManager {
    pub records: Vec<Notification>,
    pub exit_on_error: bool,
    pub catch_error: bool,
    seen_warnings: BTreeSet<WarningKey>,
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::from_env_values(
            std::env::var("NAPARI_EXIT_ON_ERROR").ok().as_deref(),
            std::env::var("NAPARI_CATCH_ERRORS").ok().as_deref(),
        )
    }
}

impl NotificationManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_env_values(exit_on_error: Option<&str>, catch_errors: Option<&str>) -> Self {
        Self {
            records: Vec::new(),
            exit_on_error: matches!(exit_on_error, Some("1" | "True")),
            catch_error: !matches!(catch_errors, Some("0" | "False")),
            seen_warnings: BTreeSet::new(),
        }
    }

    pub fn dispatch(&mut self, notification: Notification) {
        self.records.push(notification);
    }

    pub fn receive_info(&mut self, message: impl Into<String>) {
        self.dispatch(Notification::new(message, NotificationSeverity::Info));
    }

    pub fn receive_warning(
        &mut self,
        message: impl Into<String>,
        category: impl Into<String>,
        filename: impl Into<String>,
        lineno: usize,
    ) -> bool {
        let key = WarningKey {
            message: message.into(),
            category: category.into(),
            filename: filename.into(),
            lineno,
        };
        if !self.seen_warnings.insert(key.clone()) {
            return false;
        }
        self.dispatch(Notification::new(
            key.message,
            NotificationSeverity::Warning,
        ));
        true
    }
}

pub fn should_show_console_notification(
    notification: &Notification,
    console_level: NotificationSeverity,
) -> bool {
    notification.severity >= console_level
}
