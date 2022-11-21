use std::{env, str::FromStr};

use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions},
    ConnectOptions,
};
use tracing::info;

use crate::util;

// kv存储方案:
pub struct AppSqlStorage {
    pub db: SqlitePool,
}

impl AppSqlStorage {
    pub async fn default() -> Self {
        let mut _root_dir = tauri::api::path::document_dir();

        // sqlite:tmp/app.db
        let url = env::var("DATABASE_URL").unwrap_or(util::app_cfg::get_local_sqlite_url(None));
        println!("sqlite url: {}", url);

        // todo x: 仅用于创建 db 文件
        let _ = SqliteConnectOptions::from_str(&url)
            .unwrap()
            .create_if_missing(true)
            .connect()
            .await;

        let db = SqlitePoolOptions::new().min_connections(2).connect(&url).await;

        match db {
            Ok(db) => {
                info!("connect to sqlite db success");
                Self { db }
            },
            Err(e) => {
                panic!("db create error: {:?}", e);
            },
        }
    }

    pub async fn init_migrations(&mut self) {
        // todo x: 自动执行 db migrations
        let m = Migrator::new(std::path::Path::new("./migrations"))
            .await
            .expect("migrations failed");

        match m.run(&self.db).await {
            Ok(_) => {
                info!("migrations run success");
            },
            Err(e) => {
                panic!("migrations run error: {:?}", e);
            },
        }
    }
}

#[cfg(test)]
mod test {
    use tokio;

    use super::*;

    #[tokio::test]
    async fn test_sqlite() {
        let mut db = AppSqlStorage::default().await;
        db.init_migrations().await;
        println!("test sqlite success");
    }

    #[test]
    fn test2() {
        info!("hello");
        println!("test2 success");
    }
}
