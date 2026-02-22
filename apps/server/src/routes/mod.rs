pub mod health;
pub mod todo;

use crate::db::Db;
use axum::Router;
use utoipa::OpenApi;

pub use health::router as health_router;
pub use todo::router as todo_router;
pub use todo::{CreateTodo, Todo, UpdateTodo};

#[derive(OpenApi)]
#[openapi(
    paths(
        health::health_check,
        todo::list_todos,
        todo::get_todo,
        todo::create_todo,
        todo::update_todo,
        todo::delete_todo
    ),
    components(schemas(Todo, CreateTodo, UpdateTodo))
)]
pub struct ApiDoc;

pub fn router(db: Db) -> Router {
    Router::new().merge(health_router()).merge(todo_router(db))
}
