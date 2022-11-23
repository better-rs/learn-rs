use sqlx::{Acquire, Executor, Statement};

use crate::{proto::TodoEntity, storage::db::SqlConn};

pub struct TodoSqlScope {
    pub g: SqlConn,
}

impl TodoSqlScope {
    pub fn new(g: SqlConn) -> Self {
        Self { g }
    }

    pub async fn add_todo(&self, todo: &TodoEntity) -> anyhow::Result<i64> {
        let mut conn = (self.g).conn.acquire().await?;

        // Insert the task, then obtain the ID of this row
        let id = sqlx::query!(
            r#"
    INSERT INTO todos ( description, title )
    VALUES ( ?1, ?2 )
            "#,
            todo.description,
            todo.title
        )
        .execute(&mut conn)
        .await?
        .last_insert_rowid();

        Ok(id)
    }

    pub async fn list_todo(&self) -> anyhow::Result<Vec<TodoEntity>> {
        let mut conn = self.g.conn.acquire().await?;

        let rows = sqlx::query!(
            r#"
SELECT id, description, title, done, completed
FROM todos
ORDER BY id
        "#
        )
        .fetch_all(&mut conn)
        .await?;

        let mut todos = Vec::new();
        for row in rows {
            println!("row: {:?}", row);
            todos.push(TodoEntity {
                id: row.id,
                description: row.description,
                title: row.title,
                done: row.done,
                completed: row.completed,
            });
        }

        Ok(todos)
    }

    pub async fn get_todo(&self, id: i64) -> anyhow::Result<TodoEntity> {
        let mut conn = self.g.conn.acquire().await?;

        let row =
            sqlx::query!(r#"SELECT id, title, description, completed FROM todos WHERE id = ?"#, id)
                .fetch_one(&mut conn)
                .await?;

        println!("get_todo row: {:?}", row);

        Ok(TodoEntity {
            id: row.id,
            description: row.description,
            title: row.title,
            done: false,
            completed: row.completed,
        })
    }
}

#[cfg(test)]
mod test {
    use tokio;

    use super::*;

    async fn setup() -> TodoSqlScope {
        let mut db = SqlConn::new("test.db").await;
        db.init_migrations().await;

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
