use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

// Database connection
pub fn establish_connection() -> MysqlConnection {
    // parse env:
    dotenv().ok();

    // get env vars:
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // connect to db:
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
