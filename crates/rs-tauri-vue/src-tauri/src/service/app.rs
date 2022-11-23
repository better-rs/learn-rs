use crate::storage::AppStorage;

pub struct AppService {
    pub storage: AppStorage,
}

impl AppService {
    pub async fn new() -> Self {
        let storage = AppStorage::new().await;
        Self { storage }
    }
}

#[cfg(test)]
mod test {
    use crate::proto::TodoEntity;

    use super::*;

    #[tokio::test]
    async fn test_new_service() {
        let service = AppService::new().await;

        let ret = service
            .storage
            .sql
            .todo
            .add_todo(&TodoEntity {
                id: 0,
                title: "add todo, by service".to_string(),
                description: "test desc".to_string(),
                done: false,
                completed: false,
            })
            .await
            .unwrap();

        println!("service add todo: {}", ret);

        let todo = service.storage.sql.todo.get_todo(ret).await.unwrap();

        println!("service get todo: {:?}", todo);
    }
}
