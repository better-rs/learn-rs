use log::{debug, error, info, warn};
use rs_entity::{todos, todos::Entity as Todos};
use sea_orm::*; // todo x: 需要导入*, 隐含一些方法, 不导入会报错

pub struct Query;

impl Query {
    pub async fn get_todo(db: &DbConn, id: i32) -> Result<Option<todos::Model>, DbErr> {
        // todo x: clion 无法自动补全
        Todos::find_by_id(id).one(db).await
    }

    // If ok, returns (post models, num pages).
    pub async fn list_todos(db: &DbConn) -> Result<Vec<todos::Model>, DbErr> {
        // todo x: 查询全部
        Todos::find().all(db).await
    }
}
