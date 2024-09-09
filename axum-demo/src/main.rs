use axum::Json;
use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};

/// 一个合法的handler
/// 要么是axum::response::Response
/// 要么实现了axum::response::IntoResponse
/// 大多数的原生类型和axum::Json包装后的数据是axum::Response
async fn hello_world() -> &'static str {
    "Hello world!"
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(unused)]
struct User {
    pub name: String,
    pub id: i32,
}

/// 如果想返回Json对象，可以使用axum::Json包装返回结果，返回的类型必须实现类Serialize
async fn hey() -> Json<User> {
    Json(User {
        name: "cdd".to_string(),
        id: 1001,
    })
}

fn init_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/hey", get(hey))
}

#[tokio::main]
async fn main() {
    let app = init_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
