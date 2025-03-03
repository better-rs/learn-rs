use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

// macOS only
pub fn tray_menu() -> SystemTrayMenu {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item
    // label.

    let name = CustomMenuItem::new("app".to_string(), "Tauri + Vue");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");

    let about = CustomMenuItem::new("about".to_string(), "About");

    SystemTrayMenu::new()
        .add_item(name)
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(about)
}
