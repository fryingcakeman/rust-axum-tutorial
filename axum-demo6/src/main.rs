use axum::routing::get;
use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

async fn hello_world() -> &'static str {
    "Hello world!"
}

fn init_router() -> Router {
    Router::new()
        .nest_service(
            "/",
            ServeDir::new("dist").not_found_service(ServeFile::new("dist/index.html")),
        )
        .route("/hello", get(hello_world))
}

#[tokio::main]
async fn main() {
    let app = init_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
