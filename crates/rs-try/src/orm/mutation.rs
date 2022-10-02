use rs_entity::{todos, todos::Entity as Todos};
use sea_orm::{DbConn, DbErr, DeleteResult, Set, *};

pub struct Mutation;

impl Mutation {
    // todo x: 插入
    pub async fn create_todo(
        db: &DbConn,
        form_data: todos::Model,
    ) -> Result<todos::ActiveModel, DbErr> {
        todos::ActiveModel {
            description: Set(form_data.description.to_owned()),
            done: Set(form_data.done.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

    // pub async fn update_todo(
    //     db: &DbConn,
    //     id: i32,
    //     form_data: todos::Model,
    // ) -> Result<todos::Model, DbErr> {
    //     let todos: todos::ActiveModel = todos::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom("Cannot find todos.".to_owned()))
    //         .map(Into::into)?;
    //
    //     todos::ActiveModel {
    //         id: todos.id,
    //         description: Set(form_data.description.to_owned()),
    //         ..Default::default()
    //     }
    //     .update(db)
    //     .await
    // }

    // pub async fn delete_todos(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
    //     let todos: todos::ActiveModel = todos::find_by_id(id)
    //         .one(db)
    //         .await?
    //         .ok_or(DbErr::Custom("Cannot find todos.".to_owned()))
    //         .map(Into::into)?;
    //
    //     todos.delete(db).await
    // }
    //
    // pub async fn delete_all_todos(db: &DbConn) -> Result<DeleteResult, DbErr> {
    //     todos::delete_many().exec(db).await
    // }
}
