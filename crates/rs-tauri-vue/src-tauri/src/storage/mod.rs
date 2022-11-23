pub use kv::*;
pub use sql::*;

pub mod kv;
pub mod sql;

pub struct AppStorage {
    pub kv: AppKvStorage,
    // pub sql: AppSqlStorage,
}

impl AppStorage {
    pub async fn new() -> Self {
        let kv = AppKvStorage::default();
        // let sql = AppSqlStorage::new().await;
        Self { kv }
    }
}
