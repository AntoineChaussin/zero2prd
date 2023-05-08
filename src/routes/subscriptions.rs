use axum::extract::{Form, State};
use http::StatusCode;
use sqlx::PgPool;

use chrono::Utc;
use uuid::Uuid;

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, connection),
    fields(
        subscriber_email = %form.email,
        susbscriber_name = %form.name
        )
    )]
pub async fn subscription(
    State(connection): State<PgPool>,
    form: Form<FormData>,
) -> Result<StatusCode, StatusCode> {
    let insert_res = insert_subscriber(&connection, &form.0).await;

    match insert_res {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[tracing::instrument(name = "Saving in DB", skip(form, pool))]
async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    let _q_res = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {:?}", e);
        e
    })?;

    Ok(())
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
