use std::str::FromStr;

use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    ConnectOptions, SqlitePool,
};

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

pub mod util {
    use std::{env, path::PathBuf};

    pub fn local_dir(folder: Option<&str>) -> PathBuf {
        let mut current_dir = env::current_dir().expect("get current directory failed");

        let dest_dir = current_dir.join(folder.unwrap_or("tmp"));
        if !dest_dir.exists() {
            std::fs::create_dir(&dest_dir).expect("create tmp dir failed");
        }
        dest_dir
    }

    pub fn local_sqlite_url(db_name: Option<&str>) -> &str {
        let fp = db_name.unwrap_or("app.db");
        let mut prefix = "sqlite:";

        // get or create:
        let d = local_dir(None);

        // return db url
        let db_url = format!("{}{}", prefix, d.join(fp).to_str().unwrap());
        Box::leak(db_url.into_boxed_str())
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_get_local_sqlite_url() {
            let cases = [None, Some("test.db")];

            // iter
            for case in cases.iter() {
                let url = local_sqlite_url(*case);
                println!("url: {}", url);
            }
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
