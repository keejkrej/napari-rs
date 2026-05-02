use napari_rs::utils::notifications::{
    Notification, NotificationManager, NotificationSeverity, WarningNotification,
    should_show_console_notification,
};

#[test]
fn notification_severity_strings_icons_and_order_match_python_model() {
    assert_eq!(NotificationSeverity::Error.to_string(), "error");
    assert_eq!(NotificationSeverity::Warning.to_string(), "warning");
    assert_eq!(NotificationSeverity::Info.to_string(), "info");
    assert_eq!(NotificationSeverity::Debug.to_string(), "debug");
    assert_eq!(NotificationSeverity::None.to_string(), "none");

    assert_eq!(NotificationSeverity::Error.as_icon(), "ⓧ");
    assert_eq!(NotificationSeverity::Warning.as_icon(), "⚠️");
    assert_eq!(NotificationSeverity::Info.as_icon(), "ⓘ");
    assert_eq!(NotificationSeverity::Debug.as_icon(), "🐛");
    assert_eq!(NotificationSeverity::None.as_icon(), "");

    assert!(NotificationSeverity::Error > NotificationSeverity::Warning);
    assert!(NotificationSeverity::Warning > NotificationSeverity::Info);
    assert!(NotificationSeverity::Info > NotificationSeverity::Debug);
    assert!(NotificationSeverity::Debug > NotificationSeverity::None);

    assert_eq!(
        "ERROR".parse::<NotificationSeverity>().unwrap(),
        NotificationSeverity::Error
    );
    assert_eq!(
        "warning".parse::<NotificationSeverity>().unwrap(),
        NotificationSeverity::Warning
    );
    assert!("fatal".parse::<NotificationSeverity>().is_err());
}

#[test]
fn notification_string_and_console_threshold_match_python_behavior() {
    let warning = Notification::new("careful", NotificationSeverity::Warning);
    assert_eq!(warning.to_string(), "WARNING: careful");
    assert!(should_show_console_notification(
        &warning,
        NotificationSeverity::Info
    ));
    assert!(should_show_console_notification(
        &warning,
        NotificationSeverity::Warning
    ));
    assert!(!should_show_console_notification(
        &warning,
        NotificationSeverity::Error
    ));

    let action = Notification::with_actions("details", NotificationSeverity::Info, ["open"]);
    assert_eq!(action.actions, vec!["open"]);
}

#[test]
fn warning_notification_string_matches_python_shape() {
    let warning = WarningNotification::new("watch out", "UserWarning", Some("file.py"), Some(7));
    assert_eq!(warning.to_string(), "file.py:7: UserWarning: watch out!");
    assert_eq!(warning.notification.severity, NotificationSeverity::Warning);
}

#[test]
fn notification_manager_env_flags_and_warning_dedup_match_python_logic() {
    let default_flags = NotificationManager::from_env_values(None, None);
    assert!(!default_flags.exit_on_error);
    assert!(default_flags.catch_error);

    let strict = NotificationManager::from_env_values(Some("1"), Some("0"));
    assert!(strict.exit_on_error);
    assert!(!strict.catch_error);

    let mut manager = NotificationManager::from_env_values(None, None);
    manager.receive_info("ready");
    assert_eq!(manager.records.len(), 1);
    assert_eq!(manager.records[0].severity, NotificationSeverity::Info);

    assert!(manager.receive_warning("warn", "UserWarning", "file.py", 5));
    assert!(!manager.receive_warning("warn", "UserWarning", "file.py", 5));
    assert!(manager.receive_warning("warn", "UserWarning", "file.py", 6));
    assert_eq!(manager.records.len(), 3);
    assert_eq!(manager.records[1].severity, NotificationSeverity::Warning);
}
