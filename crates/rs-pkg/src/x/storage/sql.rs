use std::str::FromStr;

use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    ConnectOptions, SqlitePool,
};

use crate::util;

// db 存储方案: SqlClient
pub struct SqliteClient {
    pub cli: SqlitePool,
}

impl Clone for SqliteClient {
    fn clone(&self) -> Self {
        Self { cli: self.cli.clone() }
    }
}

impl SqliteClient {
    pub async fn default() -> Self {
        Self::new(None, None).await
    }

    pub async fn new(db_url: Option<&str>, migrations: Option<&str>) -> Self {
        // let mut _root_dir = tauri::api::path::document_dir();

        // sqlite:tmp/app.db
        let url = db_url.unwrap_or(util::local_sqlite_url(None));

        // todo x: 仅用于创建 db 文件
        let _ = SqliteConnectOptions::from_str(&url)
            .unwrap()
            .create_if_missing(true)
            .connect()
            .await
            .expect("connect to sqlite failed");

        // connect pool
        let cli = SqlitePoolOptions::new().min_connections(2).connect(&url).await;

        match cli {
            Ok(c) => {
                let dir = migrations.unwrap_or("./migrations");

                // if dir exist, do migrate
                if std::path::Path::new(dir).exists() {
                    // todo x: 自动执行 db migrations
                    let m =
                        Migrator::new(std::path::Path::new(dir)).await.expect("migrations failed");

                    match m.run(&c).await {
                        Ok(_) => {
                            println!("migrations run success");
                        },
                        Err(e) => {
                            panic!("migrations run error: {:?}", e);
                        },
                    }
                }

                // // todo x: 初始化表结构
                // sqlx::migrate!(migrations.unwrap_or("./migrations"))
                //     .run(&conn)
                //     .await
                //     .expect("migrate failed");

                Self { cli: c }
            },
            Err(e) => {
                panic!("connect to sqlite db failed: {}", e);
            },
        }
    }
}

#[cfg(test)]
mod test {
    use tokio;

    use super::*;

    #[tokio::test]
    async fn test_new_sqlite_client() {
        let db = SqliteClient::default().await;

        let cases = ["tmp/test1.db", "tmp/test2.db"];

        for item in cases.iter() {
            let _ = SqliteClient::new(Some(item), None).await;
        }
    }
}
