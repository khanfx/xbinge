use dotenv::dotenv;
use std::net::SocketAddr;
use axum::{
    routing::post,
    routing::get,
    Router,
    http::StatusCode
};
use sqlx::postgres::PgPoolOptions;

mod util;
mod schedules;

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
        .route("/schedules", post(schedules::post_schedule))
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
    Ok(util::create_id())
}
