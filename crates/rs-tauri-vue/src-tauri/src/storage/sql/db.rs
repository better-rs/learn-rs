use rs_pkg::x::SqliteClient;

use crate::storage::todo::TodoSqlScope;

// sql storage biz
pub struct AppSqlStorage {
    g: SqliteClient,

    // biz units:
    pub todo: TodoSqlScope,
}

impl AppSqlStorage {
    pub async fn new() -> Self {
        let g = SqliteClient::default().await;

        let todo = TodoSqlScope::new(g.clone());
        Self { g, todo }
    }
}
