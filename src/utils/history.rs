use std::path::{Path, PathBuf};

pub const MAX_HISTORY_LEN: usize = 10;

pub fn update_history(history: &mut Vec<PathBuf>, filename: impl AsRef<Path>) {
    let new_location = filename
        .as_ref()
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(PathBuf::new);

    if let Some(index) = history
        .iter()
        .position(|location| location == &new_location)
    {
        let existing = history.remove(index);
        history.insert(0, existing);
    } else {
        history.insert(0, new_location);
    }

    history.truncate(MAX_HISTORY_LEN);
}

pub fn get_history(history: &[PathBuf]) -> Vec<PathBuf> {
    let folders = history
        .iter()
        .filter(|folder| folder.is_dir())
        .cloned()
        .collect::<Vec<_>>();
    if folders.is_empty() {
        vec![home_dir()]
    } else {
        folders
    }
}

fn home_dir() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}
