use std::net::SocketAddr;

use dotenvy::dotenv;
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
    let socket_address: SocketAddr = addr.parse().unwrap();
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();
    println!("Server running on http://{}", socket_address);

    let db = db::connect().await.expect("Failed to connect to database");
    db::migrate(&db).await.expect("Failed to run migrations");

    let app = routes::router(db)
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        .merge(SwaggerUi::new("/swagger").url("/openapi.json", ApiDoc::openapi()));

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap()
}
