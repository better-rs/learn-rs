//! Provides a RESTful web server managing some Todos.
//!
//! API will be:
//!
//! - `GET /todos`: return a JSON list of Todos.
//! - `POST /todos`: create a new Todo.
//! - `PUT /todos/:id`: update a specific Todo.
//! - `DELETE /todos/:id`: delete a specific Todo.
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p example-todos
//! ```

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Extension, Path, Query},
    handler::Handler,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};
use dotenvy::dotenv;
// use log::{debug, info, warn};
use crate::service::hello;
// use pretty_env_logger;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, RwLock},
    time::Duration,
};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;
use uuid::Uuid;
mod service;
mod utils;
use crate::utils::{route, shutdown};
// use rs_cms_migration::{Migrator, MigratorTrait};
use sea_orm::{prelude::*, Database, QueryOrder, Set};
use tracing::{debug, error, info, warn, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "rs_cms=debug,tower_http=debug,sea_orm=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();
    // for (key, value) in env::vars() {
    //     println!("{key}: {value}");
    // }

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").unwrap_or("127.0.0.1".into());
    let port = env::var("PORT").unwrap_or("4000".into());
    let server_url = format!("{}:{}", host, port);
    debug!("db url: {}", db_url);

    // todo x: db conn:
    let conn = Database::connect(db_url).await.expect("Database connection failed");

    // todo x: migration
    // Migrator::up(&conn, None).await.unwrap();

    debug!("debug print");
    info!("info print");
    warn!("warn print");
    error!("error print");

    // fix conflict with tracing_subscriber
    // pretty_env_logger::init_custom_env("CMS_LOG");
    let db = hello::Db::default();

    // Compose the routes
    let app = Router::new()
        .fallback(route::handler_404.into_service())
        .route("/", get(hello::hello))
        .route("/index", get(hello::todos_index).post(hello::todos_create))
        .route("/todos", get(hello::todos_index).post(hello::todos_create))
        .route("/todos/:id", patch(hello::todos_update).delete(hello::todos_delete))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(Extension(db))
                .into_inner(),
        );

    // add a fallback service for handling routes to unknown paths
    // let app = app.fallback(handler_404());

    // app.fallback(handler_404());

    let addr = SocketAddr::from_str(&server_url).unwrap();

    debug!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown::shutdown_signal()) // graceful shutdown
        .await
        .unwrap();
}
