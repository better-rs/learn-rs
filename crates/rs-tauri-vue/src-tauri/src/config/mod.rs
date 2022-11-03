use std::path::PathBuf;
use tauri::api::path;

pub struct AppConfig {}

impl AppConfig {
    pub fn new() -> AppConfig {
        Self {}
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
