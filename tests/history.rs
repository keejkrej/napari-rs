use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use napari_rs::utils::history::{MAX_HISTORY_LEN, get_history, update_history};

#[test]
fn update_history_inserts_parent_folder_at_front_like_python_helpers() {
    let root = unique_temp_dir("insert");
    let folder = root.join("data");
    fs::create_dir_all(&folder).unwrap();
    let mut history = Vec::new();

    update_history(&mut history, folder.join("some-file.svg"));

    assert_eq!(history, [folder]);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn update_history_moves_existing_folder_to_front_and_truncates_to_ten() {
    let root = unique_temp_dir("truncate");
    fs::create_dir_all(&root).unwrap();
    let mut history = (0..12)
        .map(|index| {
            let folder = root.join(format!("folder-{index}"));
            fs::create_dir_all(&folder).unwrap();
            folder
        })
        .collect::<Vec<_>>();
    let existing = history[5].clone();

    update_history(&mut history, existing.join("image.tif"));

    assert_eq!(history.len(), MAX_HISTORY_LEN);
    assert_eq!(history[0], existing);
    assert_eq!(
        history
            .iter()
            .filter(|folder| *folder == &history[0])
            .count(),
        1
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn get_history_filters_to_existing_directories_or_home_fallback() {
    let root = unique_temp_dir("filter");
    let existing = root.join("existing");
    fs::create_dir_all(&existing).unwrap();
    let missing = root.join("missing");

    assert_eq!(get_history(&[missing, existing.clone()]), [existing]);

    let fallback = get_history(&[]);
    assert_eq!(fallback.len(), 1);
    assert!(!fallback[0].as_os_str().is_empty());

    fs::remove_dir_all(root).unwrap();
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "napari-rs-history-{label}-{}-{nanos}",
        std::process::id()
    ))
}
