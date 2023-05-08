use std::net::{SocketAddr, TcpListener};

use sqlx::{Connection, Executor, PgConnection, PgPool};
use zero2prd::configuration::get_config;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health", &app.address))
        .send()
        .await
        .expect("Failed to execute health check request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    let body = "name=le%20guen&email=ursulaleguen@gno.com";
    let response = client
        .post(&format!("http://{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subs");

    assert_eq!(saved.name, "le guen");
    assert_eq!(saved.email, "ursulaleguen@gno.com");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guen", "missing email"),
        ("email=patleguen@gno.com", "missing name"),
        ("", "missing everything"),
    ];

    for (invalid_body, scenario) in test_cases {
        let response = client
            .post(&format!("http://{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            422,
            response.status().as_u16(),
            "Error code not correct for scenario {}",
            scenario
        )
    }
}
async fn spawn_app() -> TestApp {
    let listner = TcpListener::bind("127.0.0.1:0".parse::<SocketAddr>().unwrap()).unwrap();

    let db_pool = setup_db().await;
    let address = listner.local_addr().unwrap();
    let server = axum::Server::from_tcp(listner).unwrap().serve(
        zero2prd::startup::app(db_pool.clone())
            .await
            .into_make_service(),
    );
    tokio::spawn(server);

    TestApp { address, db_pool }
}

async fn setup_db() -> PgPool {
    let mut configuration = get_config().expect("Failed to get config");
    configuration.database.db_name = uuid::Uuid::new_v4().to_string();

    let mut connection =
        PgConnection::connect(&configuration.database.connection_string_no_dbname())
            .await
            .expect("Failed to connect to postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, &configuration.database.db_name).as_str())
        .await
        .expect("Failed to create db");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Could not connect to postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Could not run migration");

    connection_pool
}

struct TestApp {
    address: SocketAddr,
    db_pool: PgPool,
}
