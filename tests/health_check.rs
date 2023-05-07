use std::net::{SocketAddr, TcpListener};

use sqlx::Connection;
use zero2prd::configuration::get_config;

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health", addr))
        .send()
        .await
        .expect("Failed to execute health check request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let addr = spawn_app();

    let configuration = get_config().expect("Failed to get config");

    let connection_string = configuration.database.connection_string();

    println!("{}", &connection_string);

    let mut connection = sqlx::PgConnection::connect(&connection_string)
        .await
        .expect("failed to connect to DB");

    let client = reqwest::Client::new();

    let body = "name=le%20guen&email=ursulaleguen@gno.com";
    let response = client
        .post(&format!("http://{}/subscriptions", addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subs");

    assert_eq!(saved.name, "le guen");
    assert_eq!(saved.email, "ursulaleguen@gno.com");
}

#[tokio::test]
async fn subscribe_returns_a_400_for_invalid_form_data() {
    let addr = spawn_app();

    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guen", "missing email"),
        ("email=patleguen@gno.com", "missing name"),
        ("", "missing everything"),
    ];

    for (invalid_body, scenario) in test_cases {
        let response = client
            .post(&format!("http://{}/subscriptions", addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "Error code not correct for scenario {}",
            scenario
        )
    }
}
fn spawn_app() -> SocketAddr {
    let listner = TcpListener::bind("127.0.0.1:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listner.local_addr().unwrap();
    tokio::spawn(async move {
        axum::Server::from_tcp(listner)
            .unwrap()
            .serve(zero2prd::startup::app().await.into_make_service())
            .await
            .unwrap();
    });

    addr
}
