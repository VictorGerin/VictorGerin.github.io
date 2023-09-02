use std::time::Duration;

use axum::{routing::get, Router, http::Method};
use postgres::{Client, NoTls};
use tower_http::cors::CorsLayer;
use tokio::time;

#[tokio::main]
async fn main() {

    let origins: Vec<axum::http::HeaderValue> = vec![
        "https://victorgerin.github.io".parse().unwrap(),
        "http://localhost:8080".parse().unwrap(),
    ];
    
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(origins);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(get_default))
        .route("/app", get(get_app))
        .route("/teste", get(get_teste))
        .layer(cors);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"[::]:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_default() -> String {
    "Hello, World!".to_string()
}

async fn get_teste() -> String {
    time::sleep(Duration::from_secs(5)).await;
    "Hello, World! Teste".to_string()
}

async fn get_app() -> String {
    let str_con = "postgres://postgres:zqyd2PiCxHmRCbp@muddy-leaf-3674-db.flycast:5432";
    let mut client = Client::connect(str_con, NoTls).expect("fudeu ! 0");

    client.batch_execute("
        CREATE TABLE person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BYTEA
        )
    ").expect("Fudeu ! 1");

    let name = "Ferris";
    let data = None::<&[u8]>;
    
    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&name, &data],
    ).expect("Fudeu ! 2");



    "app".to_string()
}