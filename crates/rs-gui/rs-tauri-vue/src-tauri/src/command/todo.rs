#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use tracing::info;

use crate::{proto::TodoEntity, service::AppService};

#[tauri::command]
pub async fn add_todo(title: String, description: String) -> i64 {
    let s = AppService::new().await;

    let ret = s
        .storage
        .sql
        .todo
        .add_todo(&TodoEntity { id: 0, title, description, done: false, completed: false })
        .await;

    info!("add todo: {:?}", ret);

    ret.unwrap()
}
