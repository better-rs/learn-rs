#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use tauri::{
    api::shell, CustomMenuItem, Manager, Menu, Submenu, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};

#[tauri::command]
fn backend_add(number: i32) -> i32 {
    // Note: these commands block the main thread and hang the UI until they return.
    // If you need to run a long-running task, use async command instead.
    println!("Backend was called with an argument: {}", number);
    number + 2
}

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item
    // label.
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu);

    let ctx = tauri::generate_context!();

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { position: _, size: _, .. } => {
                println!("system tray received a left click");
            },
            SystemTrayEvent::RightClick { position: _, size: _, .. } => {
                println!("system tray received a right click");
            },
            SystemTrayEvent::DoubleClick { position: _, size: _, .. } => {
                println!("system tray received a double click");
            },
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                },
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                },
                _ => {},
            },
            _ => {},
        })
        .invoke_handler(tauri::generate_handler![backend_add])
        .menu(tauri::Menu::os_default("Tauri Vue Template").add_submenu(Submenu::new(
            "Help",
            Menu::with_items([
                CustomMenuItem::new("Online Documentation", "Online Documentation").into(),
            ]),
        )))
        .on_menu_event(|event| {
            let event_name = event.menu_item_id();
            match event_name {
                "Online Documentation" => {
                    let url = "https://github.com/Uninen/tauri-vue-template".to_string();
                    shell::open(&event.window().shell_scope(), url, None).unwrap();
                },
                _ => {},
            }
        })
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                let main_window = _app.get_window("main").unwrap();
                main_window.open_devtools();
            }
            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
