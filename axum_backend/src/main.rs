use axum::{
    http::{header::CONTENT_TYPE, Method, StatusCode, Uri},
    routing::get,
    Json, Router,
};
use common::{FormContent, News};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app().await).await.unwrap();
}

async fn not_found(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

pub async fn app() -> Router {
    let origins = ["http://localhost:8080".parse().unwrap()];
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::OPTIONS,
            Method::DELETE,
        ])
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    Router::new()
        .route("/news", get(get_news))
        .layer(cors)
        .fallback(not_found)
}

async fn get_news() -> Result<Json<Vec<News>>, String> {
    Err("Error get news".to_string())
}

async fn post_appointment(Json(form_content): Json<FormContent>) -> Result<(), String> {
    Err("Error post appointment".to_string())
}
