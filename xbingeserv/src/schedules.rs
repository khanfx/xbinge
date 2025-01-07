use serde::{Deserialize, Serialize};
use axum::{
    Json,
    http::StatusCode
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Debug, Deserialize)]
pub struct CreateItem {
    name: String,
}

#[derive(Debug, Serialize)]
pub struct ItemResponse {
    id: i32,
    name: String,
}

pub async fn create_item(
    state: axum::extract::State<Pool<Postgres>>,
    Json(payload): Json<CreateItem>,
) -> Result<Json<ItemResponse>, (StatusCode, String)> {

    let mut conn = state.acquire().await.map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to connect to database: {}", e),
        )
    })?;


    let name = payload.name;

    // Insert the item into the database
    let row = sqlx::query!(
        "INSERT INTO items (name) VALUES ($1) RETURNING id, name",
        name
    )

    // Results in error:
    // the trait `Executor<'_>` is not implemented for `&mut sqlx::pool::PoolConnection<Postgres>`
    //.fetch_one(&mut conn)
    .fetch_one(conn.as_mut())

    .await
    .map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to insert item: {}", e),
        )
    })?;

    Ok(Json(ItemResponse {
        id: row.id,
        name: row.name,
    }))
}
