use tauri::{api::process::restart, AppHandle, Env, Manager, Runtime, Wry};

// 重启 app:
pub fn restart_app(env: &Env) {
    restart(env);
}

// 显示 app 主窗口:
pub fn show_main_window(app: &AppHandle<Wry>) {
    let win = app.get_window("main").unwrap();
    if win.is_visible().unwrap() {
        win.hide().unwrap();
    } else {
        // win.hide().unwrap();
        win.set_focus().unwrap();
    }
}

pub fn open_browser(url: &str) {}
