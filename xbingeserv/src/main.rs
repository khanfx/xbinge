use axum::{
    routing::post,
    routing::get,
    Json, Router,
    http::StatusCode
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use dotenv::dotenv;
use rand::Rng;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};



#[derive(Debug, Deserialize)]
struct CreateItem {
    name: String,
}

#[derive(Debug, Serialize)]
struct ItemResponse {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Create a connection pool to PostgreSQL
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Share the connection pool across handlers
    let app = Router::new()
        .route("/items", post(create_item))
        .route("/rand", get(get_rand))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_rand() -> Result<String, (StatusCode, String)> {
    let mut rng = rand::thread_rng();
    let hex_string: String = (0..16)
        .map(|_| format!("{:x}", rng.gen_range(0..16)))
        .collect();
    Ok(hex_string)
}

async fn create_item(
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
