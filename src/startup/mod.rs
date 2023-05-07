use crate::configuration::get_config;
use crate::routes::{health_check, subscription};
use axum::{routing::get, routing::post, Router};
use std::error::Error;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let configuration = get_config().expect("Failed to read config");
    axum::Server::bind(
        &format!("0.0.0.0:{}", configuration.application_port)
            .parse()
            .unwrap(),
    )
    .serve(app().await.into_make_service())
    .await?;
    Ok(())
}

pub async fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/health", get(health_check))
        .route("/subscriptions", post(subscription))
}
