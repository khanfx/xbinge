use serde::{Deserialize, Serialize};
use axum::{
    Json,
    http::StatusCode
};
use sqlx::{Pool, Postgres};

#[derive(Debug, Deserialize)]
pub struct Episode {
    id: String,
    name: String,
    watch_date: String,
}

#[derive(Debug, Deserialize)]
pub struct PostScheduleRequest {
    id: String,
    name: String,
    episodes: Option<Vec<Episode>>,
}

#[derive(Debug, Serialize)]
pub struct ScheduleResponse {
    id: String,
    name: String,
}

pub async fn post_schedule(
    state: axum::extract::State<Pool<Postgres>>,
    Json(payload): Json<PostScheduleRequest>,
) -> Result<Json<ScheduleResponse>, (StatusCode, String)> {

    println!("Processing post_schedule request {payload:#?}");
    dbg!(&state);
    dbg!(&payload);

    let mut conn = state.acquire().await.map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to connect to database: {}", e),
        )
    })?;

    let row = sqlx::query!("SELECT COUNT(*) AS c FROM schedules where id=$1", payload.id)
        .fetch_one(conn.as_mut())
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to check if schedule exists: {}", e),
            )
        })?;

    if row.c != Some(0)
    {
        sqlx::query!(
            "UPDATE schedules SET name=$2 where id=$1",
            payload.id,
            payload.name
        ).execute(conn.as_mut());
        println!("Updated schedule: {:?}", payload);
    }
    else
    {
        let row = sqlx::query!(
            "INSERT INTO schedules (id, name) VALUES ($1, $2) RETURNING id, name",
            payload.id,
            payload.name
        )

        // Results in error:
        // the trait `Executor<'_>` is not implemented for `&mut sqlx::pool::PoolConnection<Postgres>`
        //.fetch_one(&mut conn)
        .fetch_one(conn.as_mut())

        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to insert schedule: {}", e),
            )
        })?;
        println!("Inserted schedule: {:?}", row);
    }

    Ok(Json(ScheduleResponse {
        id: payload.id,
        name: payload.name,
    }))
}
