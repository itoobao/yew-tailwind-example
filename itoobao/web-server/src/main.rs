mod errors;
mod middlewares;
mod models;
mod state;
use axum::{
    extract::{self, extractor_middleware, Extension},
    handler::get,
    http::Request,
    AddExtensionLayer, Router,
};
use std::net::SocketAddr;
use tower::util::AndThenLayer;

use crate::{
    middlewares::{customer_response, SupportRequestMethod},
    models::{DbPool, User},
    state::AppState,
};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "web_server=debug")
    }
    tracing_subscriber::fmt::init();
    //db连接
    tracing::debug!("connect database ...start...");
    let share_state = match AppState::new().await {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    tracing::debug!("connect database ...end...");
    let app = Router::new()
        .route("/", get(root))
        //中间件-before 处理
        .layer(extractor_middleware::<SupportRequestMethod>())
        //中间件-after 处理
        .layer(AndThenLayer::new(customer_response))
        .layer(AddExtensionLayer::new(share_state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(state: extract::Extension<AppState>) -> &'static str {
    tracing::debug!("/");
    let user = User {
        uid: 1,
        username: String::from("admin"),
        pwd: String::from("abc"),
    };
    user.get_user(&state.db_pool).await;
    "hello world"
}
