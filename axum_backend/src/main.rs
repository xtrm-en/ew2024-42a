use axum::{
    http::{header::CONTENT_TYPE, Method, StatusCode, Uri},
    routing::{get, post},
    Json, Router,
};
use common::{FormContent, News, NewsType};
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
        .route("/appointment", post(post_appointment))
        .layer(cors)
        .fallback(not_found)
}

async fn get_news() -> Result<Json<Vec<News>>, String> {
    Ok(Json(vec![
        News {
            title: "Exam".to_string(),
            content: "Exam 02, 03, 04, 05, 06".to_string(),
            r#type: NewsType::Intra42,
        },
        News {
            title: "HTBM".to_string(),
            content: "Internship and Apprenticeship".to_string(),
            r#type: NewsType::StaffMsg,
        },
        News {
            title: "Conference YY Company".to_string(),
            content: "YY need to hire new students".to_string(),
            r#type: NewsType::CampusEvent,
        },
        News {
            title: "commodo magna cillum elit".to_string(),
            content: "sed nostrud".to_string(),
            r#type: NewsType::Intra42,
        },
        News {
            title: "ut in reprehenderit eiusmod nisi".to_string(),
            content: "ut nostrud laboris consectetur".to_string(),
            r#type: NewsType::CampusEvent,
        },
        News {
            title: "anim reprehenderit".to_string(),
            content: "nulla tempor pariatur Excepteur".to_string(),
            r#type: NewsType::StaffMsg,
        },
        News {
            title: "Ut exercitation aliquip nostrud".to_string(),
            content: "id sed sint dolore".to_string(),
            r#type: NewsType::Intra42,
        },
        News {
            title: "mollit consectetur culpa veniam".to_string(),
            content: "officia consequat mollit".to_string(),
            r#type: NewsType::CampusEvent,
        },
        News {
            title: "laboris".to_string(),
            content: "fugiat occaecat".to_string(),
            r#type: NewsType::StaffMsg,
        },
        News {
            title: "Joy a réalisé un salto dans la cour de l'école !!!".to_string(),
            content: "Si vous avez raté cette évènement, pas de chance...".to_string(),
            r#type: NewsType::StaffMsg,
        },
    ]))
}

async fn post_appointment(Json(_form_content): Json<FormContent>) -> Result<(), String> {
    Ok(())
}
