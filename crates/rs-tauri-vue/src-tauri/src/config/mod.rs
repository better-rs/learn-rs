use std::path::PathBuf;

use tauri::api::path;

// 解析系统配置文件, 加载配置
pub struct AppConfig {
    // 应用配置文件
    pub config_file: String,

    // 应用kv存储:
    pub kv_storage_file: String,

    // 应用加密kv存储:
    pub encrypted_kv_storage_file: String,

    // 应用 sql 存储:
    pub sql_storage_file: String,
}

impl AppConfig {
    pub fn default() -> AppConfig {
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
    pub fn config_dir(&self) -> Option<PathBuf> {
        path::config_dir()
    }

    pub fn document_dir(&self) -> Option<PathBuf> {
        path::document_dir()
    }

    pub fn cache_dir(&self) -> Option<PathBuf> {
        path::cache_dir()
    }

    pub fn data_dir(&self) -> Option<PathBuf> {
        path::data_dir()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cfg = AppConfig::default();

        println!("config_dir: {:?}", cfg.config_dir());
        println!("document_dir: {:?}", cfg.document_dir());
        println!("cache_dir: {:?}", cfg.cache_dir());
        println!("data_dir: {:?}", cfg.data_dir());

        println!("The current locale is {:?}", cfg.to_string());
    }
}
