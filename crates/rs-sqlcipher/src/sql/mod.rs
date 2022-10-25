use sqlx::{sqlite::SqlitePool, Executor, SqliteConnection};
use sqlx_rt::fs::File;
use std::{env, path::PathBuf};
use structopt::StructOpt;

pub async fn new_db_url() -> anyhow::Result<(String, PathBuf)> {
    let path = env::current_dir()?;
    let filepath = path.join("tmp/app.db");

    // Touch the file, so DB driver will not complain it does not exist
    File::create(filepath.as_path()).await?;

    Ok((format!("sqlite://{}", filepath.display()), path))
}

#[derive(StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(StructOpt)]
pub enum Command {
    Add { description: String },
    Add2 { description: String },
    Done { id: i64 },
}

pub async fn add_todo(pool: &SqlitePool, description: String) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;

    // Insert the task, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
INSERT INTO todos ( description )
VALUES ( ?1 )
        "#,
        description
    )
    .execute(&mut conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

pub async fn add_todo2(conn: &mut SqliteConnection, description: String) -> anyhow::Result<i64> {
    let id = sqlx::query!(
        r#"
INSERT INTO todos ( description )
VALUES ( ?1 )
        "#,
        description
    )
    .execute(conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

pub async fn complete_todo(pool: &SqlitePool, id: i64) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
UPDATE todos
SET done = TRUE
WHERE id = ?1
        "#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

pub async fn list_todos(pool: &SqlitePool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!("- [{}] {}: {}", if rec.done { "x" } else { " " }, rec.id, &rec.description,);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::sql::new_db_url;
    use std::env;

    #[sqlx_macros::test]
    async fn it_works() -> anyhow::Result<()> {
        let path = env::current_dir()?;
        println!("The current directory is {}", path.display());

        let (url, _dir) = new_db_url().await?;

        println!("db url is {}", url);

        let result = 2 + 2;
        assert_eq!(result, 4);

        Ok(())
    }
}
