use axum::extract::{Form, State};
use http::StatusCode;
use sqlx::PgPool;

use chrono::Utc;
use uuid::Uuid;

pub async fn subscription(
    State(connection): State<PgPool>,
    form: Form<FormData>,
) -> Result<StatusCode, StatusCode> {
    let q_res = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&connection)
    .await;

    match q_res {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
