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

pub mod app_cfg {
    use std::{env, path::PathBuf};

    pub fn get_local_tmp_dir() -> PathBuf {
        let tmp_local = "tmp";
        env::current_dir().expect("get current directory failed").join(tmp_local)
    }

    pub fn get_or_create_local_tmp_dir() -> PathBuf {
        let tmp_local = "tmp";
        let mut current_dir = env::current_dir().expect("get current directory failed");
        let tmp_dir = current_dir.join(tmp_local);
        if !tmp_dir.exists() {
            std::fs::create_dir(&tmp_dir).expect("create tmp dir failed");
        }
        tmp_dir
    }

    pub fn get_local_sqlite_url(db_name: Option<&str>) -> String {
        let fp = db_name.unwrap_or("app.db");
        let prefix = "sqlite:";
        format!("{}{}", prefix, get_or_create_local_tmp_dir().join(fp).to_str().unwrap())
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_get_local_sqlite_url() {
            let url = get_local_sqlite_url(None);
            println!("local sqlite url: {}", url);

            let url = get_local_sqlite_url(Some("test.db"));
            println!("local sqlite url: {}", url);
        }
    }
}
