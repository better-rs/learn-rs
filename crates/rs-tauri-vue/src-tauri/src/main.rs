#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use command::calc::backend_add;
use tauri::{
    api::shell, CustomMenuItem, Manager, Menu, Runtime, Submenu, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem, Window,
};
use tracing::{debug, info, info_span, trace_span, warn};
use window_shadows::set_shadow;

mod command;
mod menu;
mod sql;

fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE).init();

    // tips:
    info!("tauri main started");

    // todo x: 系统托盘菜单
    let tray = SystemTray::new().with_menu(menu::tray_menu());

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
        .menu(tauri::Menu::os_default("CryptoPie").add_submenu(Submenu::new(
            "Help",
            Menu::with_items([
                CustomMenuItem::new("Online Documentation", "Online Documentation").into(),
            ]),
        )))
        .menu(menu::menu())
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
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let main_window = app.get_window("main").unwrap();
                main_window.open_devtools();
            }

            let win = app.get_window("main").unwrap();

            // todo x: 更改透明窗口
            #[cfg(target_os = "macos")]
            win.set_transparent_title_bar(false, false);

            set_shadow(&win, true).unwrap();
            // 监听更新消息
            win.listen("tauri://update-status".to_string(), move |msg| {
                println!("New status: {:?}", msg);
            });

            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowStyleMask, NSWindowTitleVisibility};

pub trait WindowExt {
    #[cfg(target_os = "macos")]
    fn set_transparent_title_bar(&self, title_transparent: bool, remove_toolbar: bool);
}

impl<R: Runtime> WindowExt for Window<R> {
    #[cfg(target_os = "macos")]
    fn set_transparent_title_bar(&self, title_transparent: bool, remove_tool_bar: bool) {
        unsafe {
            let id = self.ns_window().unwrap() as cocoa::base::id;
            NSWindow::setTitlebarAppearsTransparent_(id, cocoa::base::YES);
            let mut style_mask = id.styleMask();
            style_mask.set(NSWindowStyleMask::NSFullSizeContentViewWindowMask, title_transparent);

            if remove_tool_bar {
                style_mask.remove(
                    NSWindowStyleMask::NSClosableWindowMask
                        | NSWindowStyleMask::NSMiniaturizableWindowMask
                        | NSWindowStyleMask::NSResizableWindowMask,
                );
            }

            id.setStyleMask_(style_mask);

            id.setTitleVisibility_(if title_transparent {
                NSWindowTitleVisibility::NSWindowTitleHidden
            } else {
                NSWindowTitleVisibility::NSWindowTitleVisible
            });

            id.setTitlebarAppearsTransparent_(if title_transparent {
                cocoa::base::YES
            } else {
                cocoa::base::NO
            });
        }
    }
}
