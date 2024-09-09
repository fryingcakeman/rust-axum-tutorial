use axum::extract::Request;
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use axum::routing::get;
use axum::{middleware, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::compression::CompressionLayer;

#[derive(Clone, Debug)]
#[allow(unused)]
pub struct AppState {
    db: PgPool,
}

async fn hello_world() -> &'static str {
    "Hello world"
}

/// 实现字节的middleware
async fn check_hello_world(req: Request, next: Next) -> Result<Response, StatusCode> {
    if req.headers().get(CONTENT_TYPE).unwrap() != "application/json" {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(next.run(req).await)
}

/// 可以使用任何的tower中间件
async fn init_router() -> Router {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("db_url")
        .await
        .unwrap();
    let state = Arc::new(AppState { db: pool });

    Router::new()
        .route("/", get(hello_world))
        .layer(CompressionLayer::new())
        .layer(middleware::from_fn(check_hello_world)) // 添加自定义middleware
        .layer(middleware::from_fn_with_state(
            // 给中间件添加应用状态
            state.clone(),
            check_hello_world,
        ))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let app = init_router().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
