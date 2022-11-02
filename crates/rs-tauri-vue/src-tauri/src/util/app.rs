use tauri::{api::process::restart, Wry};

pub fn app_restart(app: tauri::Builder<Wry>) {
    restart(&app.env());
}
