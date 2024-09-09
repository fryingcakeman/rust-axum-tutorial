use axum::extract::{FromRequest, Path, Query, Request};
use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{async_trait, Form, Json, RequestExt, Router};
use axum_extra::headers::Origin;
use axum_extra::TypedHeader;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct User {
    id: i32,
    name: String,
}

/// 提取器extractor：从http请求中获取各种参数
#[allow(unused)]
async fn get_user(
    Json(user): Json<User>, // 如果传入User结构中没有的参数，将会被忽略
    Form(form): Form<User>,
    Path(name): Path<String>,
    Query(id): Query<i32>,
) {
    println!("{:?}", user);
}

/// 处理Header需要用axum-extra包
/// axum-extra还有处理cookie、protobuf等的功能
#[allow(unused)]
async fn my_handler(TypedHeader(origin): TypedHeader<Origin>) {
    println!("{}", origin)
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    foo: String,
}

#[allow(unused)]
async fn handler(JsonOrForm(payload): JsonOrForm<Payload>) {
    dbg!(payload);
}

struct JsonOrForm<T>(T);

/// 实现自己的提取器
#[async_trait]
impl<S, T> FromRequest<S> for JsonOrForm<T>
where
    S: Send + Sync,
    Json<T>: FromRequest<()>,
    Form<T>: FromRequest<()>,
    T: 'static,
{
    type Rejection = Response;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                let Json(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self(payload));
            }

            if content_type.starts_with("application/x-www-form-urlencoded") {
                let Form(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self(payload));
            }
        }

        Err(StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response())
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/handler", get(handler))
        .route("/myhandler", get(my_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
