use crate::ctx::AppContext;

pub struct AppService {
    pub ctx: &'static AppContext,
}

impl AppService {
    pub async fn new() -> Self {
        let ctx = AppContext::global().await;
        Self { ctx }
    }
}
