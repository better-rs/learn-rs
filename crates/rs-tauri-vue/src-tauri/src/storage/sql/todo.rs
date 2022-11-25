use sqlx::{Acquire, Executor, Row, Statement};
use tracing::info;

use rs_pkg::x::SqliteClient;

use crate::proto::TodoEntity;

pub struct TodoSqlScope {
    pub g: SqliteClient,
}

impl TodoSqlScope {
    pub fn new(g: SqliteClient) -> Self {
        Self { g }
    }

    pub async fn add_todo(&self, todo: &TodoEntity) -> anyhow::Result<i64> {
        let mut conn = self.g.cli.acquire().await?;

        // Insert the task, then obtain the ID of this row
        let id = sqlx::query(
            r#"
        INSERT INTO todos ( description, title )
        VALUES ( ?1, ?2 )
                "#,
        )
        .bind(&todo.description)
        .bind(&todo.title)
        .execute(&mut conn)
        .await?
        .last_insert_rowid();

        info!("add todo: {:?}", todo);

        Ok(id)
    }

    pub async fn list_todo(&self) -> anyhow::Result<Vec<TodoEntity>> {
        let mut conn = self.g.cli.acquire().await?;

        let rows = sqlx::query(
            r#"
    SELECT id, description, title, done, completed
    FROM todos
    ORDER BY id
            "#,
        )
        .fetch_all(&mut conn)
        .await?;

        let mut todos = Vec::new();
        for row in rows {
            // println!("row: {:?}", row);
            todos.push(TodoEntity {
                id: row.get(0),
                description: row.get(1),
                title: row.get(2),
                done: row.get(3),
                completed: row.get(4),
            });
        }

        Ok(todos)
    }

    pub async fn get_todo(&self, id: i64) -> anyhow::Result<TodoEntity> {
        let mut conn = self.g.cli.acquire().await?;

        let row = sqlx::query(
            r#"SELECT id, title, description, completed FROM todos WHERE id =
    ?"#,
        )
        .bind(id)
        .fetch_one(&mut conn)
        .await?;

        // println!("get_todo row: {:?}", row);

        Ok(TodoEntity {
            id: row.get("id"),
            description: row.get("description"),
            title: row.get("title"),
            done: false,
            completed: row.get("completed"),
        })
    }
}

#[cfg(test)]
mod test {
    use tokio;

    use super::*;

    async fn setup() -> TodoSqlScope {
        let mut db = SqliteClient::new(None, None).await;
        // let mut db = SqlConn::default().await;
        // db.init_migrations().await;

        println!("💖💖💖💖💖 setup");

        TodoSqlScope::new(db)
    }

    #[tokio::test]
    async fn test_add_todo() {
        let db = setup().await;
        let todo = TodoEntity {
            id: 0,
            title: "test title".to_string(),
            description: "test desc".to_string(),
            done: false,
            completed: false,
        };
        let id = db.add_todo(&todo).await.unwrap();
        assert!(id > 0);
    }

    #[tokio::test]
    async fn test_get_todo() {
        let db = setup().await;
        let todo = TodoEntity {
            id: 0,
            title: "get todo test".to_string(),
            description: "test desc".to_string(),
            done: false,
            completed: false,
        };
        let id = db.add_todo(&todo).await.unwrap();
        let todo = db.get_todo(id).await.unwrap();
        assert_eq!(todo.description, "test desc");
    }

    #[tokio::test]
    async fn test_list_todo() {
        let db = setup().await;
        let todo = TodoEntity {
            id: 0,
            title: "test title".to_string(),
            description: "test desc".to_string(),
            done: false,
            completed: false,
        };
        let id = db.add_todo(&todo).await.unwrap();
        assert!(id > 0);

        let todos = db.list_todo().await.unwrap();

        println!("todos: {:?}", todos)
    }
}
