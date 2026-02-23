use std::net::SocketAddr;

use axum::Router;
use dotenvy::dotenv;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod db;
mod routes;

use routes::ApiDoc;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let addr = std::env::var("ADDR").expect("ADDR env variable is required");
    let cors_origin = std::env::var("CORS_ORIGIN").expect("CORS_ORIGIN env variable is required");
    let socket_address: SocketAddr = addr.parse().unwrap();
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();
    println!("Server running on http://{}", socket_address);

    let db = db::connect().await.expect("Failed to connect to database");
    db::migrate(&db).await.expect("Failed to run migrations");

    let cors = CorsLayer::new()
        .allow_origin(cors_origin.parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers([axum::http::header::ACCEPT, axum::http::header::CONTENT_TYPE])
        .allow_credentials(true);

    let app = Router::new()
        .merge(routes::router(db))
        .layer(cors)
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .merge(SwaggerUi::new("/swagger").url("/openapi.json", ApiDoc::openapi()));

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
