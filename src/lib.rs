use std::error::Error;

use axum::{routing::get, Router};
use http::StatusCode;

pub async fn run() -> Result<(),Box<dyn Error>> {
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app().await.into_make_service())
        .await?;
    Ok(())
}

pub async fn app() -> Router {
    Router::new()
        .route("/", get(|| async {"Hello World"}))
        .route("/health",get(health_check))
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
