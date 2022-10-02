use log::{info, warn};
use sea_orm::{ConnectOptions, Database};

use rs_try::orm::{mutation::Mutation, query::Query};

use std::{env, error::Error, time::Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    // let db = Database::connect(&env::var("DATABASE_URL")?).await.unwrap();
    let db_url = &env::var("DATABASE_URL")?;

    // todo x: 配置 db 连接参数
    let mut opt = ConnectOptions::new(db_url.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    // todo x: 自带连接池 https://www.sea-ql.org/SeaORM/docs/install-and-config/connection/
    let db = Database::connect(opt).await?;

    info!("db url:{:?}, conn {:?}\n", db_url, db);

    // todo x: 查询
    let ret = Query::get_todo(&db, 1).await?;

    info!("todo get: {:?}", ret);

    Ok(())
}
