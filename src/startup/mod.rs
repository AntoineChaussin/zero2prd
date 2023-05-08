use crate::configuration::get_config;
use crate::routes::{health_check, subscription};
use axum::{routing::get, routing::post, Router};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::error::Error;
use tower::ServiceBuilder;
use tower_http::request_id::MakeRequestUuid;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

use tower_http::ServiceBuilderExt;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub async fn run() -> Result<(), Box<dyn Error>> {
    let configuration = get_config().expect("Failed to read config");
    let connection = PgPool::connect(&configuration.database.connection_string().expose_secret())
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
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_response(DefaultOnResponse::new().include_headers(true)),
                )
                .propagate_x_request_id(),
        )
}

pub fn setup_log<Sink>(logger: String, default_level: String, sink: Sink)
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level));

    let formatting_layer = BunyanFormattingLayer::new(logger, sink);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber).expect("Cannot set default log level");
}
