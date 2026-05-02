use std::cell::Cell;
use std::rc::Rc;

use napari_rs::utils::task_status::{Status, TaskStatusManager};

#[test]
fn status_strings_match_python_string_enum_values() {
    assert_eq!(Status::Pending.to_string(), "pending");
    assert_eq!(Status::Busy.to_string(), "busy");
    assert_eq!(Status::Completed.to_string(), "completed");
    assert_eq!(Status::Cancelled.to_string(), "cancelled");
    assert_eq!(Status::Failed.to_string(), "failed");
    assert_eq!(Status::StartFailed.to_string(), "start_failed");
    assert_eq!("BUSY".parse(), Ok(Status::Busy));
}

#[test]
fn task_status_registration_update_and_busy_messages_match_python_behavior() {
    let mut manager = TaskStatusManager::new();
    let cancelled = Rc::new(Cell::new(false));
    let cancelled_for_callback = Rc::clone(&cancelled);

    let task_status_id = manager.register_task_status(
        "test-task-status",
        Status::Busy,
        "Register task status busy",
        Some(Box::new(move || {
            cancelled_for_callback.set(true);
            true
        })),
    );

    assert_eq!(manager.len(), 1);
    assert!(manager.is_busy());
    assert_eq!(
        manager.get_status(),
        ["test-task-status: Register task status busy"]
    );

    assert!(manager.update_task_status(
        task_status_id,
        Status::Completed,
        "Register task status completed",
    ));
    assert!(!manager.is_busy());
    assert!(manager.get_status().is_empty());

    assert!(manager.update_task_status(
        task_status_id,
        Status::Pending,
        "Register task status pending",
    ));
    assert!(manager.is_busy());
    assert_eq!(
        manager.get_status(),
        ["test-task-status: Register task status pending"]
    );

    manager.cancel_all();

    assert!(cancelled.get());
    assert!(manager.get_status().is_empty());
    assert!(!manager.is_busy());
    assert_eq!(
        manager.get(task_status_id).unwrap().status_history().last(),
        Some(&Status::Cancelled)
    );
}

#[test]
fn update_unknown_task_returns_false_like_python_manager() {
    let mut manager = TaskStatusManager::new();
    let task_status_id = manager.register_task_status("provider", Status::Busy, "running", None);

    assert!(!manager.update_task_status(
        napari_rs::utils::task_status::TaskStatusId::from(task_status_id.get() + 1),
        Status::Completed,
        "done",
    ));
    assert!(manager.is_busy());
}
