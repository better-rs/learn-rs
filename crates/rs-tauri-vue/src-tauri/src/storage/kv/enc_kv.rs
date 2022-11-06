use microkv::MicroKV;
use std::path::{Path, PathBuf};

// 加密kv存储方案:
pub struct AppEncryptedKVStorage {
    db: MicroKV,
}

impl AppEncryptedKVStorage {
    pub fn default() -> AppEncryptedKVStorage {
        // TODO X: 默认使用 用户文档目录
        let root_dir = tauri::api::path::document_dir().unwrap_or("./tmp/".parse().unwrap());
        let db_name = "app.test.kv.db";
        let unsafe_pwd = "unsafe_pwd";

        let db: MicroKV = MicroKV::open_with_base_path(db_name, root_dir)
            .expect("Failed to create MicroKV from a stored file or create MicroKV for this file")
            .set_auto_commit(true)
            .with_pwd_clear(unsafe_pwd);

        Self { db }
    }

    pub fn new(root_dir: PathBuf, db_name: &str, unsafe_pwd: &str) -> Self {
        let db: MicroKV = MicroKV::open_with_base_path(db_name, root_dir)
            .expect("Failed to create MicroKV from a stored file or create MicroKV for this file")
            .set_auto_commit(true)
            .with_pwd_clear(unsafe_pwd);

        println!("Creating MicroKV: {:?}", db.exists("key1"));

        Self { db }
    }

    pub fn set_locale(&mut self, locale: &str) -> microkv::errors::Result<()> {
        let key = "app:locale";

        self.db.put(key, &locale)
    }

    pub fn get_locale(&self) -> String {
        let key = "app:locale";

        self.db.get_unwrap(key).unwrap_or("en".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn it_works() {
        let mut kv = AppEncryptedKVStorage::default();

        kv.set_locale("zh-CN").expect("set locale error: panic message");

        let ret = kv.get_locale();
        println!("get locale: {:?}", ret);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_new() {
        let mut dir = env::temp_dir();
        dir.push("microkv");

        println!("current directory: {:?}", dir);

        let mut kv = AppEncryptedKVStorage::new(dir, "app.test.kv.json", "test pwd");

        kv.set_locale("zh-HK").expect("set locale error:");

        let ret = kv.get_locale();
        println!("get locale: {:?}", ret);
    }
}
