use sqlx::{Acquire, Executor, Statement};

use crate::storage::db::AppSqlStorage;

impl AppSqlStorage {
    //     pub async fn add_todo(&self, todo: &TodoEntity) -> anyhow::Result<i64> {
    //         let mut conn = self.db.acquire().await?;
    //
    //         // Insert the task, then obtain the ID of this row
    //         let id = sqlx::query!(
    //             r#"
    // INSERT INTO todos ( description )
    // VALUES ( ?1 )
    //         "#,
    //             description
    //         )
    //         .execute(&mut conn)
    //         .await?
    //         .last_insert_rowid();
    //
    //         Ok(id)
    //     }
}

#[cfg(test)]
mod test {
    use tokio;

    use super::*;

    async fn setup() -> AppSqlStorage {
        let mut db = AppSqlStorage::new("test.db").await;
        db.init_migrations().await;
        db
    }
}
