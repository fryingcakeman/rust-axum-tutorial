use axum::extract::{FromRef, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[allow(unused)]
enum ApiResponse {
    OK,
    Created,
    JsonData(Vec<Message>),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

#[allow(unused)]
enum ApiError {
    BadRequest,
    Forbidden,
    Unauthorised,
    InternalServerError,
    UserDefineError,
}

/// 自定义的错误类型实现IntoResponse
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST).into_response(),
            Self::Forbidden => (StatusCode::FORBIDDEN).into_response(),
            Self::Unauthorised => (StatusCode::UNAUTHORIZED).into_response(),
            Self::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            Self::UserDefineError => ("User define error").into_response(),
        }
    }
}

/// 使用应用程序状态:一个专门用于保存在应用程序上的路由之间共享的所有变量的结构
/// 在Axum中执行此操作的唯一要求是该结构需要实现Clone
#[derive(Clone)]
#[allow(unused)]
struct AppState {
    db: PgPool,
    api_state: ApiState,
}

/// 子状态
#[derive(Clone)]
struct ApiState {}

impl FromRef<AppState> for ApiState {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.api_state.clone()
    }
}

async fn hello_world() -> &'static str {
    "Hello world"
}

/// 要使用它，将其插入到路由器中，并将状态作为参数传递到函数中
#[allow(unused)]
async fn do_something(State(state): State<Arc<AppState>>) -> Result<ApiResponse, ApiError> {
    // 使用state
    todo!()
}

fn init_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/do_something", get(do_something))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("db_url")
        .await
        .unwrap();

    // let state = AppState { db: pool };

    // 还可以将应用程序状态结构包装在原子引用计数器（std::sync::Arc）中，而不是使用#[derive(Clone)]
    let state = Arc::new(AppState {
        db: pool,
        api_state: ApiState {},
    });

    let app = init_router(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
