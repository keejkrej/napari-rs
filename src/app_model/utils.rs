use crate::app_model::constants::{MenuId, menu_group};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuCommand {
    pub id: String,
}

impl MenuCommand {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MenuItem {
    pub command: Option<MenuCommand>,
}

impl MenuItem {
    pub fn command(id: impl Into<String>) -> Self {
        Self {
            command: Some(MenuCommand::new(id)),
        }
    }

    pub const fn submenu() -> Self {
        Self { command: None }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DummyAction {
    pub id: String,
    pub title: String,
    pub menu_id: String,
    pub group: String,
    pub when: String,
    pub enablement: bool,
    pub palette: bool,
}

pub fn to_id_key(menu_path: &str) -> &str {
    menu_path.rsplit('/').next().unwrap_or(menu_path)
}

pub fn to_action_id(id_key: &str) -> String {
    format!("napari.{id_key}.empty_dummy")
}

pub fn contains_dummy_action(menu_items: &[MenuItem]) -> bool {
    menu_items.iter().any(|item| {
        item.command
            .as_ref()
            .is_some_and(|command| command.id.contains("empty_dummy"))
    })
}

pub fn is_empty_menu(menu_items: Option<&[MenuItem]>) -> bool {
    match menu_items {
        None => true,
        Some([]) => true,
        Some([item]) => contains_dummy_action(std::slice::from_ref(item)),
        Some(_) => false,
    }
}

pub fn get_dummy_action(menu_id: MenuId) -> (DummyAction, String) {
    let id_key = to_id_key(menu_id.as_str());
    let context_key = format!("{id_key}_empty");
    let action = DummyAction {
        id: to_action_id(id_key),
        title: "Empty".to_string(),
        menu_id: menu_id.to_string(),
        group: menu_group::NAVIGATION.to_string(),
        when: context_key.clone(),
        enablement: false,
        palette: false,
    };
    (action, context_key)
}
