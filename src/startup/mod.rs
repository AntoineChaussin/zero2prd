use crate::configuration::get_config;
use crate::routes::{health_check, subscription};
use axum::{routing::get, routing::post, Router};
use sqlx::PgPool;
use std::error::Error;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let configuration = get_config().expect("Failed to read config");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to DB");
    axum::Server::bind(
        &format!("0.0.0.0:{}", configuration.application_port)
            .parse()
            .unwrap(),
    )
    .serve(app(connection).await.into_make_service())
    .await?;
    Ok(())
}

pub async fn app(connection: PgPool) -> Router {
    Router::new()
        .route("/subscriptions", post(subscription))
        .with_state(connection)
        .route("/", get(|| async { "Hello World" }))
        .route("/health", get(health_check))
}
