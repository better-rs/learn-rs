use std::path::PathBuf;
use tauri::api::path;

// 解析系统配置文件, 加载配置
pub struct AppConfig {
    // 应用配置文件
    config_file: String,

    // 应用kv存储:
    kv_storage_file: String,

    // 应用加密kv存储:
    encrypted_kv_storage_file: String,

    // 应用 sql 存储:
    sql_storage_file: String,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        Self {
            config_file: String::from("app.config.json"),
            kv_storage_file: String::from("app.kv.db"),
            encrypted_kv_storage_file: "app.kv.enc.db".to_string(),
            sql_storage_file: "app.sql.db".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "AppConfig: cfg={}, kv={}, kv_enc={}, sql={}.",
            self.config_file,
            self.kv_storage_file,
            self.encrypted_kv_storage_file,
            self.sql_storage_file
        )
    }

    pub fn get_locale() {}

    pub fn set_locale(&mut self, locale: &str) {}

    // 配置文件路径:
    pub fn config_dir() -> Option<PathBuf> {
        path::config_dir()
    }

    pub fn document_dir() -> Option<PathBuf> {
        path::document_dir()
    }

    pub fn cache_dir() -> Option<PathBuf> {
        path::cache_dir()
    }

    pub fn data_dir() -> Option<PathBuf> {
        path::data_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cfg = AppConfig::new();

        println!("The current locale is {:?}", cfg.to_string());
    }
}
