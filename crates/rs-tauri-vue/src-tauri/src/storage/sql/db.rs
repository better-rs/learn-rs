use std::str::FromStr;

use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions},
    Acquire, ConnectOptions, Executor,
};
use tracing::info;

use crate::{storage::todo::TodoSqlScope, util};

// sql storage biz
pub struct AppSqlStorage {
    g: SqlConn,

    // biz units:
    pub todo: TodoSqlScope,
}

impl AppSqlStorage {
    pub async fn new() -> Self {
        let g = SqlConn::default().await;

        // g.init_migrations().await;

        let todo = TodoSqlScope::new(g.clone());
        Self { g, todo }
    }
}

// kv存储方案: SqlConn
pub struct SqlConn {
    pub conn: SqlitePool,
}

impl Clone for SqlConn {
    fn clone(&self) -> Self {
        Self { conn: self.conn.clone() }
    }
}

impl SqlConn {
    pub async fn default() -> Self {
        let conn = new_sql_conn("app.db").await;
        Self { conn }
    }

    pub async fn new(db_name: &str) -> Self {
        let conn = new_sql_conn(db_name).await;
        Self { conn }
    }

    pub async fn init_migrations(&mut self) {
        // todo x: 自动执行 db migrations
        let m = Migrator::new(std::path::Path::new("./migrations"))
            .await
            .expect("migrations failed");

        match m.run(&self.conn).await {
            Ok(_) => {
                println!("migrations run success");
            },
            Err(e) => {
                panic!("migrations run error: {:?}", e);
            },
        }
    }
}

async fn new_sql_conn(db_name: &str) -> SqlitePool {
    let mut _root_dir = tauri::api::path::document_dir();

    // sqlite:tmp/app.db
    // let url = env::var("").unwrap_or(util::app_cfg::get_local_sqlite_url(Some(db_name)));
    let url = util::app_cfg::get_local_sqlite_url(Some(db_name));
    println!("sqlite url: {}", url);

    // todo x: 仅用于创建 db 文件
    let _ = SqliteConnectOptions::from_str(&url)
        .unwrap()
        .create_if_missing(true)
        .connect()
        .await
        .expect("connect to sqlite failed");

    // connect pool
    let conn = SqlitePoolOptions::new().min_connections(2).connect(&url).await;

    match conn {
        Ok(conn) => {
            // todo x: 初始化表结构
            sqlx::migrate!("./migrations").run(&conn).await.expect("migrate failed");

            info!("connect to sqlite db success");
            conn
        },
        Err(e) => {
            panic!("connect to sqlite db failed: {}", e);
        },
    }
}

#[cfg(test)]
mod test {
    use tokio;

    use super::*;

    #[tokio::test]
    async fn test_sqlite() {
        let mut db = SqlConn::default().await;
        // db.init_migrations().await;

        println!("test sqlite success");
    }

    #[test]
    fn test2() {
        info!("hello");
        println!("test2 success");
    }
}
