use std::{env, str::FromStr};

use sqlx::{
    migrate::Migrator,
    sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions},
    Acquire, ConnectOptions, Executor,
};
use tracing::info;

use crate::{storage::todo::TodoSqlScope, util};

// sql storage biz

pub struct AppSqlStorageBiz {
    pub g: SqlConn,

    // biz units:
    pub todo: TodoSqlScope,
}

impl AppSqlStorageBiz {
    pub async fn new() -> anyhow::Result<Self> {
        let g = SqlConn::default().await?;

        let todo = TodoSqlScope::new(g.clone());

        Ok(Self { g, todo })
    }
}

// kv存储方案: SqlConn
pub struct SqlConn {
    pub conn: SqlitePool,
}

impl SqlConn {
    pub async fn default() -> Self {
        let mut _root_dir = tauri::api::path::document_dir();

        // sqlite:tmp/app.db
        let url = env::var("").unwrap_or(util::app_cfg::get_local_sqlite_url(None));
        println!("sqlite url: {}", url);

        // todo x: 仅用于创建 db 文件
        let _ = SqliteConnectOptions::from_str(&url)
            .unwrap()
            .create_if_missing(true)
            .connect()
            .await;

        let conn = SqlitePoolOptions::new().min_connections(2).connect(&url).await;

        match conn {
            Ok(conn) => {
                info!("sqlite conn: {:?}", conn);
                Self { conn }
            },
            Err(e) => {
                panic!("sqlite conn err: {:?}", e);
            },
        }
    }

    pub async fn new(db_name: &str) -> Self {
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
            .await;

        let db = SqlitePoolOptions::new().min_connections(2).connect(&url).await;

        match db {
            Ok(db) => {
                info!("connect to sqlite db success");
                Self { conn: db }
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

        match m.run(&self.conn).await {
            Ok(_) => {
                println!("migrations run success");
            },
            Err(e) => {
                panic!("migrations run error: {:?}", e);
            },
        }
    }

    // pub async fn add_todo2(&self, todo: &TodoEntity) -> anyhow::Result<i64> {
    //     let mut conn = self.db.acquire().await?;
    //
    //     // let id = sqlx::query!(r#""#).execute(&mut conn).await?.last_insert_rowid();
    //
    //     let id = 2;
    //     Ok(id)
    // }
    //
    // pub async fn get_todo(&self, id: i64) -> anyhow::Result<TodoEntity> {
    //     let mut conn = self.db.acquire().await?;
    //     let mut tx = conn.begin().await?;
    //     let mut stmt = tx.prepare(r#"SELECT id, title, completed FROM todo WHERE id =
    // ?"#).await?;     let mut rows = stmt.query(sqlx::params![id]).await?;
    //     let row = rows.next().await.unwrap()?;
    //     tx.commit().await?;
    //
    //     Ok(Todo { id: row.get(0), title: row.get(1), completed: row.get(2) })
    // }
    //
    // pub async fn list_todos(&mut self) -> anyhow::Result<Vec<TodoEntity>> {
    //     let mut conn = self.db.acquire().await?;
    //     let mut tx = conn.begin().await?;
    //     let mut stmt = tx.prepare(r#"SELECT id, title, completed FROM todo"#).await?;
    //     let mut rows = stmt.query(sqlx::params![]).await?;
    //
    //     let mut todos = Vec::new();
    //     for row in rows {
    //         todos.push(Todo { id: row.get(0), title: row.get(1), completed: row.get(2) });
    //     }
    //     tx.commit().await?;
    //
    //     Ok(todos)
    // }
    //
    // pub async fn update_todo(&self, todo: &TodoEntity) -> anyhow::Result<bool> {
    //     let mut conn = self.db.acquire().await?;
    //     let mut tx = conn.begin().await?;
    //     let mut stmt =
    //         tx.prepare(r#"UPDATE todo SET title = ?, completed = ? WHERE id = ?"#).await?;
    //     let rows_affected =
    //         stmt.execute(sqlx::params![todo.title, todo.completed, todo.id]).await?;
    //     tx.commit().await?;
    //
    //     Ok(rows_affected > 0)
    // }
}

#[cfg(test)]
mod test {
    use tokio;

    use super::*;

    #[tokio::test]
    async fn test_sqlite() {
        let mut db = SqlConn::default().await;
        db.init_migrations().await;

        // let mut conn = db.db.acquire().await.unwrap();
        // let mut tx = conn.begin().await.unwrap();
        // // insert todo

        println!("test sqlite success");
    }

    #[test]
    fn test2() {
        info!("hello");
        println!("test2 success");
    }
}
