use tauri::{AboutMetadata, Menu, MenuItem, Submenu};

// macOS only
pub fn menu() -> Menu {
    Menu::new().add_submenu(submenu_app()).add_submenu(submenu_edit())
}

fn submenu_app() -> Submenu {
    let about_menu = AboutMetadata::new()
        .version(String::from("0.1.0"))
        .authors(vec![String::from("Henry Huang")])
        .comments(String::from("Crypto Currency Watcher"))
        .copyright(String::from("GPL-3.0 License"))
        .license(String::from("GPL-3.0 License"))
        .website(String::from("https://github.com/better-rs/learn-rs"))
        .website_label(String::from("Source Code"));

    Submenu::new(
        "CryptoWatcher",
        Menu::new()
            .add_native_item(MenuItem::About("CryptoWatcher".to_string(), about_menu))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    )
}

fn submenu_edit() -> Submenu {
    Submenu::new(
        "Edit",
        Menu::new()
            .add_native_item(MenuItem::Undo)
            .add_native_item(MenuItem::Redo)
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Cut)
            .add_native_item(MenuItem::Copy)
            .add_native_item(MenuItem::Paste)
            .add_native_item(MenuItem::SelectAll),
    )
}
