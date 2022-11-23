use std::sync::Arc;

use once_cell::sync::OnceCell;
use parking_lot::Mutex;

use crate::{
    config::AppConfig,
    storage::{db::SqlConn, AppEncryptedKVStorage, AppKvStorage},
};

#[derive(Clone)]
pub struct AppContext {
    /*
    todo x: 单例模式
        参考: https://github.com/zzzgydi/clash-verge/blob/main/src-tauri/src/data/mod.rs#L16

    */
    // 配置
    pub config: Arc<Mutex<AppConfig>>,

    // kv 存储:
    pub kv: Arc<Mutex<AppKvStorage>>,

    // kv 加密存储:
    pub kv_enc: Arc<Mutex<AppEncryptedKVStorage>>,

    // sqlite3
    pub sql: Arc<Mutex<SqlConn>>,
}

impl AppContext {
    // 单例模式:
    pub async fn global() -> &'static AppContext {
        static DATA: OnceCell<AppContext> = OnceCell::new();

        let sql = SqlConn::default().await;

        DATA.get_or_init(|| AppContext {
            config: Arc::new(Mutex::new(AppConfig::default())),
            kv: Arc::new(Mutex::new(AppKvStorage::default())),
            kv_enc: Arc::new(Mutex::new(AppEncryptedKVStorage::default())),
            sql: Arc::new(Mutex::new(sql)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let ctx = AppContext::global().await;

        let mut kv = ctx.kv.lock();

        kv.set_locale("fr").expect("TODO: panic message");
        let ret = kv.get_locale();
        println!("ctx get_locale: {:?}", ret);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
