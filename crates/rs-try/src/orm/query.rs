use rs_entity::{todos, todos::Entity as Todos};
use sea_orm::{DbConn, DbErr, *};
pub struct Query;

impl Query {
    pub async fn find_post_by_id(db: &DbConn, id: i32) -> Result<Option<todos::Model>, DbErr> {
        // Todos::find_by_id(id).one(db).await

        Ok(None)
    }

    // If ok, returns (post models, num pages).
    // pub async fn find_posts_in_page(
    //     db: &DbConn,
    //     page: u64,
    //     posts_per_page: u64,
    // ) -> Result<(Vec<post::Model>, u64), DbErr> {
    //     // Setup paginator
    //     let paginator = Post::find().order_by_asc(post::Column::Id).paginate(db, posts_per_page);
    //     let num_pages = paginator.num_pages().await?;
    //
    //     // Fetch paginated posts
    //     paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    // }
}
