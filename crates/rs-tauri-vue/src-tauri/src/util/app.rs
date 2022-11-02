use tauri::{api::process::restart, Env, Manager, Wry};
pub fn restart_app(env: &Env) {
    restart(env);
}
