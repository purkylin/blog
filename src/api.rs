use crate::state::SharedState;
use anyhow::Context;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, post},
    Router,
};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use tower_http::validate_request::ValidateRequestHeaderLayer;

pub fn routes() -> Router<SharedState> {
    let token = std::env::var("API_TOKEN").unwrap();
    let auth = ValidateRequestHeaderLayer::bearer(&token);

    Router::new()
        .route(
            "/",
            post(create_post)
                .delete(delete_post)
                .layer(auth.clone())
                .get(list_posts),
        )
        .route(
            "/:id",
            delete(delete_post)
                .post(edit_post)
                .layer(auth.clone())
                .get(get_post),
        )
}

async fn create_post(
    State(state): State<SharedState>,
    Json(post): Json<CreatePost>,
) -> AppResult<impl IntoResponse> {
    if post.title.trim().is_empty() || post.body.trim().is_empty() {
        Err(anyhow::Error::msg("Some fields is empty"))?
    }

    let mut conn = state.pool.acquire().await?;

    let id: i32 = sqlx::query_scalar("INSERT INTO post(title, body) values($1, $2) RETURNING id")
        .bind(post.title)
        .bind(post.body)
        .fetch_one(&mut conn)
        .await?;

    let ret = json!({
        "id": id,
    });

    Ok(Json(ret))
}

async fn delete_post(
    State(state): State<SharedState>,
    Path(id): Path<i32>,
) -> AppResult<impl IntoResponse> {
    let mut conn = state.pool.acquire().await?;
    let row = sqlx::query("DELETE FROM post WHERE id = $1")
        .bind(id)
        .execute(&mut conn)
        .await?;

    let status = row.rows_affected() > 0;
    let ret = json!({
        "status": status,
    });

    Ok(Json(ret))
}

async fn list_posts(
    State(state): State<SharedState>,
    Query(paginator): Query<Paginator>,
) -> impl IntoResponse {
    let mut conn = state.pool.acquire().await.unwrap();
    let rows =
        sqlx::query_as::<_, Post>("SELECT * from post ORDER BY created_at DESC LIMIT $1 OFFSET $2")
            .bind(paginator.page_size)
            .bind(paginator.offset())
            .fetch_all(&mut conn)
            .await
            .unwrap();

    Json(rows)
}

async fn get_post(
    State(state): State<SharedState>,
    Path(id): Path<i32>,
) -> AppResult<impl IntoResponse> {
    let mut conn = state.pool.acquire().await?;
    let row = sqlx::query_as::<_, Post>("SELECT * FROM post WHERE id = $1")
        .bind(id)
        .fetch_optional(&mut conn)
        .await?
        .context("not found the post")?;
    Ok(Json(row))
}

async fn edit_post(
    State(state): State<SharedState>,
    Path(id): Path<i32>,
    Json(post): Json<CreatePost>,
) -> AppResult<impl IntoResponse> {
    if post.title.trim().is_empty() || post.body.trim().is_empty() {
        Err(anyhow::Error::msg("Some fields is empty"))?
    }

    let mut conn = state.pool.acquire().await?;
    let row = sqlx::query("UPDATE post SET title=$1, body=$2, modified_at=$3 WHERE id = $4")
        .bind(post.title)
        .bind(post.body)
        .bind(Utc::now())
        .bind(id)
        .execute(&mut conn)
        .await?;

    let status = row.rows_affected() > 0;
    let ret = json!({
        "status": status,
    });

    Ok(Json(ret))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CreatePost {
    title: String,
    body: String,
    // seperate with comma
    tags: Option<String>,
}

#[derive(Serialize, FromRow)]
struct Post {
    id: i32,
    title: String,
    body: String,
    created_at: NaiveDateTime,
    modified_at: NaiveDateTime,
}

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Deserialize, Clone, Copy)]
struct Paginator {
    #[serde(default = "Paginator::default_page_size")]
    page_size: i32,
    #[serde(default = "Paginator::default_page")]
    page: i32,
}

impl Paginator {
    fn default_page_size() -> i32 {
        10
    }

    fn default_page() -> i32 {
        1
    }

    fn offset(self) -> i32 {
        let index = self.page.max(1) - 1;
        index * self.page_size
    }
}
