use napari_rs::app_model::constants::{MenuId, menu_group};
use napari_rs::app_model::utils::{
    MenuItem, contains_dummy_action, get_dummy_action, is_empty_menu, to_action_id, to_id_key,
};

#[test]
fn id_key_and_action_id_match_python_helpers() {
    assert_eq!(
        to_id_key("napari/file/open_with_plugin"),
        "open_with_plugin"
    );
    assert_eq!(to_id_key("napari/file"), "file");
    assert_eq!(to_id_key("plain"), "plain");
    assert_eq!(to_id_key("napari/file/"), "");

    assert_eq!(
        to_action_id("io_utilities"),
        "napari.io_utilities.empty_dummy"
    );
    assert_eq!(to_action_id(""), "napari..empty_dummy");
}

#[test]
fn dummy_action_detection_matches_python_command_id_substring_check() {
    let menu_items = vec![
        MenuItem::submenu(),
        MenuItem::command("napari.file.open"),
        MenuItem::command("plugin.empty_dummy_action"),
    ];
    assert!(contains_dummy_action(&menu_items));

    let real_items = vec![
        MenuItem::submenu(),
        MenuItem::command("napari.file.open"),
        MenuItem::command("plugin.action"),
    ];
    assert!(!contains_dummy_action(&real_items));
}

#[test]
fn empty_menu_detection_matches_python_cases_without_global_registry() {
    assert!(is_empty_menu(None));
    assert!(is_empty_menu(Some(&[])));
    assert!(is_empty_menu(Some(&[MenuItem::command(
        "napari.layers.empty_dummy"
    )])));
    assert!(!is_empty_menu(Some(&[MenuItem::command(
        "napari.layers.real"
    )])));
    assert!(!is_empty_menu(Some(&[
        MenuItem::command("napari.layers.empty_dummy"),
        MenuItem::command("napari.layers.real"),
    ])));
}

#[test]
fn dummy_action_metadata_matches_python_get_dummy_action() {
    let (action, context_key) = get_dummy_action(MenuId::FileIoUtilities);

    assert_eq!(context_key, "io_utilities_empty");
    assert_eq!(action.id, "napari.io_utilities.empty_dummy");
    assert_eq!(action.title, "Empty");
    assert_eq!(action.menu_id, "napari/file/io_utilities");
    assert_eq!(action.group, menu_group::NAVIGATION);
    assert_eq!(action.when, "io_utilities_empty");
    assert!(!action.enablement);
    assert!(!action.palette);
}
