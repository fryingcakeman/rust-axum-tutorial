use axum::routing::get;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router,
};
use serde::Serialize;

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

/// 可以实现IntoResponse，这样就能返回我们的自定义类型
impl IntoResponse for ApiResponse {
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

async fn do_something() -> ApiResponse {
    ApiResponse::OK
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

async fn hey() -> Result<ApiResponse, ApiError> {
    Err(ApiError::InternalServerError)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hey", get(hey))
        .route("/do", get(do_something));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
