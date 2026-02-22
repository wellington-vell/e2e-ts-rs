use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;
use utoipa::ToSchema;
use vld::prelude::*;
use vld_utoipa::impl_to_schema;

#[derive(Serialize, Deserialize, ToSchema, FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

vld::schema! {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct CreateTodo {
        pub title: String => vld::string().min(5).max(255),
    }
}
impl_to_schema!(CreateTodo);

vld::schema! {
    #[derive(Debug, Serialize, Deserialize)]
    pub struct UpdateTodo {
        pub title: Option<String> => vld::string().min(5).max(255).optional(),
        pub completed: Option<bool> => vld::boolean().optional(),
    }
}
impl_to_schema!(UpdateTodo);

pub fn router(db: PgPool) -> Router {
    Router::new()
        .route("/todos", get(list_todos))
        .route("/todos", post(create_todo))
        .route("/todos/{id}", get(get_todo))
        .route("/todos/{id}", put(update_todo))
        .route("/todos/{id}", delete(delete_todo))
        .with_state(db)
}

#[utoipa::path(
    get,
    path = "/todos",
    tag = "Todo",
    responses(
        (status = 200, description = "List all todos", body = [Todo])
    )
)]
async fn list_todos(State(db): State<PgPool>) -> Result<impl IntoResponse, impl IntoResponse> {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos")
        .fetch_all(&db)
        .await;

    match todos {
        Ok(todos) => Ok(Json(todos)),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[utoipa::path(
    get,
    path = "/todos/{id}",
    tag = "Todo",
    responses(
        (status = 200, description = "Get a todo by id", body = Todo),
        (status = 404, description = "Todo not found")
    )
)]
async fn get_todo(
    State(db): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let todo = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(&db)
        .await;

    match todo {
        Ok(Some(todo)) => Ok(Json(todo)),
        Ok(None) => Err((
            axum::http::StatusCode::NOT_FOUND,
            "Todo not found".to_string(),
        )),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[utoipa::path(
    post,
    path = "/todos",
    tag = "Todo",
    request_body = CreateTodo,
    responses(
        (status = 201, description = "Create a new todo", body = Todo)
    )
)]
async fn create_todo(
    State(db): State<PgPool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let payload = match CreateTodo::parse(&payload) {
        Ok(p) => p,
        Err(e) => return Err((axum::http::StatusCode::BAD_REQUEST, e.to_string())),
    };

    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title, completed) VALUES ($1, false) RETURNING id, title, completed",
    )
    .bind(&payload.title)
    .fetch_one(&db)
    .await;

    match todo {
        Ok(todo) => Ok((axum::http::StatusCode::CREATED, Json(todo))),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[utoipa::path(
    put,
    path = "/todos/{id}",
    tag = "Todo",
    request_body = UpdateTodo,
    responses(
        (status = 200, description = "Update a todo", body = Todo),
        (status = 404, description = "Todo not found")
    )
)]
async fn update_todo(
    State(db): State<PgPool>,
    Path(id): Path<i64>,
    Json(payload): Json<serde_json::Value>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let payload = match UpdateTodo::parse(&payload) {
        Ok(p) => p,
        Err(e) => return Err((axum::http::StatusCode::BAD_REQUEST, e.to_string())),
    };

    let todo = sqlx::query_as::<_, Todo>(
        "UPDATE todos SET title = COALESCE($1, title), completed = COALESCE($2, completed) WHERE id = $3 RETURNING id, title, completed",
    )
    .bind(&payload.title)
    .bind(payload.completed)
    .bind(id)
    .fetch_optional(&db)
    .await;

    match todo {
        Ok(Some(todo)) => Ok(Json(todo)),
        Ok(None) => Err((
            axum::http::StatusCode::NOT_FOUND,
            "Todo not found".to_string(),
        )),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    tag = "Todo",
    responses(
        (status = 204, description = "Delete a todo"),
        (status = 404, description = "Todo not found")
    )
)]
async fn delete_todo(
    State(db): State<PgPool>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&db)
        .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => Ok(axum::http::StatusCode::NO_CONTENT),
        Ok(_) => Err((
            axum::http::StatusCode::NOT_FOUND,
            "Todo not found".to_string(),
        )),
        Err(e) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
