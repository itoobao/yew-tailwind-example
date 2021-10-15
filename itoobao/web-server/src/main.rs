mod middlewares;
use axum::{extract::extractor_middleware, handler::get, Router};
use std::net::SocketAddr;
use tower::util::AndThenLayer;

use crate::middlewares::{customer_response, SupportRequestMethod};

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "web_server=debug")
    }
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        //中间件-before 处理
        .layer(extractor_middleware::<SupportRequestMethod>())
        //中间件-after 处理
        .layer(AndThenLayer::new(customer_response));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    tracing::debug!("/");
    "hello world"
}
