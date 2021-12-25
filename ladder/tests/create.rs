use std::env;

use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_create_account() {
    dotenv::dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let url = format!("http://{}:{}/create", host, port);

    let client = Client::new();

    let json = json!({
        "name": "harry",
        "username": "harry",
        "password": "admin"
    });
    let response = client
        .post(url)
        .json(&json)
        .send()
        .await
        .expect("request failed");

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_create_team() {
    dotenv::dotenv().ok();

    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let url = format!("http://{}:{}/create", host, port);

    let client = Client::new();

    let json = json!({
        "name": "harry",
        "username": "harry",
        "password": "admin"
    });
    let response = client
        .post(url)
        .json(&json)
        .send()
        .await
        .expect("request failed");

    assert_eq!(response.status(), 200);
}
