pub struct AppService {
    // pub storage: AppStorage,
}

impl AppService {
    pub async fn new() -> Self {
        // let storage = AppStorage::new().await;
        // Self { storage }
        Self {}
    }

    // pub async fn add_todo(&self, todo: &TodoEntity) -> anyhow::Result<i64> {
    //     self.storage.sql.todo.add_todo(todo).await
    // }
    //
    // pub async fn get_todo(&self, id: i64) -> anyhow::Result<TodoEntity> {
    //     self.storage.sql.todo.get_todo(id).await
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    // async fn setup() {
    //     // let mut db = SqlConn::new("app.db").await;
    //     let mut db = SqlConn::default().await;
    //
    //     db.init_migrations().await;
    // }

    // #[tokio::test]
    // async fn test_new_service() {
    //     setup().await;
    //
    //     let service = AppService::new().await;
    //
    //     let ret = service
    //         .storage
    //         .sql
    //         .todo
    //         .add_todo(&TodoEntity {
    //             id: 0,
    //             title: "add todo, by service".to_string(),
    //             description: "test desc".to_string(),
    //             done: false,
    //             completed: false,
    //         })
    //         .await
    //         .unwrap();
    //
    //     println!("service add todo: {}", ret);
    //
    //     let todo = service.storage.sql.todo.get_todo(ret).await.unwrap();
    //
    //     println!("service get todo: {:?}", todo);
    // }

    // #[tokio::test]
    // async fn test_service_add_todo() {
    //     let service = AppService::new().await;
    //     let ret = service
    //         .add_todo(&TodoEntity {
    //             id: 0,
    //             title: "add todo, by service api".to_string(),
    //             description: "test desc".to_string(),
    //             done: false,
    //             completed: false,
    //         })
    //         .await
    //         .unwrap();
    //     println!("service add todo: {}", ret);
    //
    //     let todo = service.get_todo(ret).await.unwrap();
    //     println!("service get todo: {:?}", todo);
    // }
}
