use rs_sqlcipher::sql::{add_todo, add_todo2, complete_todo, list_todos, Args, Command};

use sqlx::{
    migrate::Migrate,
    sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions},
    ConnectOptions,
};
use std::{env, str::FromStr};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::from_args_safe()?;

    // todo x: 单个 db 连接
    let mut conn = SqliteConnectOptions::from_str(&env::var("DATABASE_URL")?)?
        // .pragma("key", "the_password") // todo x: 关键参数
        .connect()
        .await?;

    // todo x: 自动执行 db migrations
    sqlx::migrate!("./migrations").run(&mut conn).await?;

    // todo x: 写法2
    let _ = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    // todo x: 基于 db 连接池
    let pool: SqlitePool = SqlitePoolOptions::new()
        .min_connections(2)
        .connect(&env::var("DATABASE_URL")?)
        .await
        .unwrap();

    // todo x: 写法2
    let _ = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    match args.cmd {
        // todo x: 新增
        Some(Command::Add { description }) => {
            println!("Adding new todo with description '{}'", &description);
            let todo_id = add_todo(&pool, description).await?;
            println!("Added new todo with id {}", todo_id);
        },

        Some(Command::Add2 { description }) => {
            println!("Adding new todo with description '{}'", &description);
            let todo_id = add_todo2(&mut conn, description).await?;
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

    // clean:
    pool.close();
    Ok(())
}
