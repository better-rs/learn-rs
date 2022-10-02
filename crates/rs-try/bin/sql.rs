use rs_try::sql::{add_todo, complete_todo, list_todos, Args, Command};

use sqlx::sqlite::SqlitePool;
use std::env;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::from_args_safe()?;
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    match args.cmd {
        // todo x: 新增
        Some(Command::Add { description }) => {
            println!("Adding new todo with description '{}'", &description);
            let todo_id = add_todo(&pool, description).await?;
            println!("Added new todo with id {}", todo_id);
        },

        // todo x: 更新状态
        Some(Command::Done { id }) => {
            println!("Marking todo {} as done", id);
            if complete_todo(&pool, id).await? {
                println!("Todo {} is marked as done", id);
            } else {
                println!("Invalid id {}", id);
            }
        },

        // todo x: 查询
        None => {
            println!("Printing list of all todos");
            list_todos(&pool).await?;
        },
    }

    Ok(())
}
