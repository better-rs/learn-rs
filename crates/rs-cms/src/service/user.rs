use axum::{
    extract::{Extension, Form, Path, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, get_service, post},
    Json, Router, Server,
};
use axum_macros::debug_handler;

use sea_orm::*;
use serde::{Deserialize, Serialize};
use tracing::debug;

use rs_entity::user;
use user::Entity as User;

pub struct UserService {}

impl UserService {
    fn new() {}

    pub async fn get_user(&self) {}

    pub async fn add_user(&self) {}
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct UserAddReq {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct UserAddResp {
    message: String,
    status: i32,
}

pub async fn add_user(
    Extension(ref conn): Extension<DatabaseConnection>,
    // form: Form<UserAddReq>,
) -> impl IntoResponse {
    // let model = form.0;

    let model = UserAddReq {
        username: "test1".to_string(),
        email: "test1@example.com".to_string(),
        password: "test password".to_string(),
    };

    // todo x: db insert
    user::ActiveModel {
        username: Set(model.username.to_owned()),
        email: Set(model.email.to_owned()),
        password: Set(model.password.to_owned()),

        ..Default::default()
    }
    .save(conn)
    .await
    .expect("could not insert post");

    let resp = UserAddResp { message: "user add ok ".to_string(), status: 200 };

    debug!("resp={:?}", resp);

    Json(resp)
}

#[derive(Deserialize)]
pub struct Params {
    page: Option<usize>,
    posts_per_page: Option<usize>,
}

// #[debug_handler] // todo x: debug
pub async fn get_user(
    Extension(ref conn): Extension<DatabaseConnection>,
    // Query(params): Query<Params>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    // let page = params.page.unwrap_or(1);
    // let posts_per_page = params.posts_per_page.unwrap_or(5);

    let qs: user::Model =
        User::find_by_id(id).one(conn).await.expect("could not find post").unwrap();

    let resp = UserResp::from(qs);

    Json(resp)
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct UserResp {
    pub user_id: u32,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl UserResp {
    pub fn from(mut entity: user::Model) -> UserResp {
        UserResp {
            user_id: entity.id,
            username: entity.username,
            email: entity.email,
            password: entity.password,
        }
    }
}
