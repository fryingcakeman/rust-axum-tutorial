use std::env;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

// 实现了axum::response::IntoResponse的类型才能作为axum handler函数的返回值
async fn hello_world() -> &'static str {
    "Hello world"
}

// 可以通过axum::Json<T>包装数据来返回json
async fn get_some_json() -> Json<Value> {
    let j = json!({"hello": "world"});
    Json(j)
}

// 如果要返回Json结构体数据，结构体需要实现Serialize
#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: i64,
    name: String,
}

async fn get_user() -> Json<User> {
    let user = User {
        id: 1001,
        name: "rust".to_string(),
    };
    Json(user)
}

// Extractors的作用
// 1.共享数据
// 2.从header中提取数据
// 3.从request中提取数据
async fn func_with_extractors(Json(user): Json<User>) -> impl IntoResponse {
    format!("The contents of user is: {:?}", user)
}

// Axum中state需要实现Clone，如果state不能（完全）实现，可以使用std::sync::Arc包装
#[derive(Clone)]
struct AppState {
    db: PgPool,
}

async fn query_from_db(State(state): State<AppState>) -> StatusCode {
    sqlx::query("SELECT id, name FROM users")
        .execute(&state.db)
        .await
        .unwrap();
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let db_url = env!("DATABASE_URL");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .unwrap();
    let state = AppState { db: pool };

    // 创建router
    let router = Router::new()
        .route_service(
            "/",
            ServeDir::new("static").not_found_service(ServeFile::new("static/index.html")),
        )
        .route("/hello", get(hello_world))
        .route("/json", get(get_some_json))
        .route("/user", get(get_user))
        .route("/extractor", post(func_with_extractors))
        .route("/query", get(query_from_db))
        .with_state(state);

    // 定义地址和端口
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    // 绑定到listener
    let listener = TcpListener::bind(&addr).await.unwrap();

    // 启动
    axum::serve(listener, router).await.unwrap();
}
