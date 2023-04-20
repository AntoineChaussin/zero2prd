use std::net::{TcpListener, SocketAddr};

#[tokio::test]
async fn health_check_works(){
    let addr = spawn_app();

    let client = reqwest::Client::new();
    
    let response=  client.get(format!("http://{}/health",addr)).send().await.expect("Failed to execute health check request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}

fn spawn_app() -> SocketAddr {
    let listner = TcpListener::bind("127.0.0.1:3000".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listner.local_addr().unwrap();
    tokio::spawn(async move {
        axum::Server::from_tcp(listner)
            .unwrap()
            .serve(zero2prd::app().await.into_make_service())
            .await
            .unwrap();
    } );

    addr
}
