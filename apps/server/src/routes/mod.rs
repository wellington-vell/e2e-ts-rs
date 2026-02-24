pub mod health;
pub mod todo;

use crate::db::Db;
use axum::Router;
use utoipa::OpenApi;

pub use health::router as health_router;
pub use health::HealthResponse;
pub use todo::router as todo_router;
pub use todo::{CreateTodo, Todo, UpdateTodo};

#[derive(OpenApi)]
#[openapi(
    paths(
        health::check,
        todo::get_all,
        todo::get_by_id,
        todo::create,
        todo::update,
        todo::destroy
    ),
    components(schemas(Todo, CreateTodo, UpdateTodo, HealthResponse))
)]
pub struct ApiDoc;

pub fn router(db: Db) -> Router {
    Router::new().merge(health_router()).merge(todo_router(db))
}
