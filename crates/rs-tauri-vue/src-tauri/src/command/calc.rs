#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use tauri::{
    api::shell, CustomMenuItem, Manager, Menu, Runtime, Submenu, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem, Window,
};
use tracing::{debug, info, info_span, trace_span, warn};

use window_shadows::set_shadow;

#[tauri::command]
pub fn backend_add(number: i32) -> i32 {
    // Note: these commands block the main thread and hang the UI until they return.
    // If you need to run a long-running task, use async command instead.
    println!("Backend was called with an argument: {}", number);
    number + 2
}
