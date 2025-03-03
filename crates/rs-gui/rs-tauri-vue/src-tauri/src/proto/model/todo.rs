#[derive(Debug)]
pub struct TodoEntity {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub done: bool,
    pub completed: bool,
}
