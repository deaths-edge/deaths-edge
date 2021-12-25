mod authorize;
pub mod models;
mod setup;

use authorize::authorize;
use tracing::{info, warn};

use std::{env, net::SocketAddr, time::Duration};

use axum::{routing::post, Router};
use sea_orm::Database;

// async fn ws_handler(
//     ws: WebSocketUpgrade,
//     user_agent: Option<TypedHeader<headers::UserAgent>>,
// ) -> impl IntoResponse {
//     if let Some(TypedHeader(user_agent)) = user_agent {
//         println!("`{}` connected", user_agent.as_str());
//     }

//     ws.on_upgrade(handle_socket)
// }

// async fn handle_socket(mut socket: WebSocket) {
//     if let Some(msg) = socket.recv().await {
//         if let Ok(msg) = msg {
//             println!("Client says: {:?}", msg);
//         } else {
//             println!("client disconnected");
//             return;
//         }
//     }

//     loop {
//         if socket
//             .send(Message::Text(String::from("Hi!")))
//             .await
//             .is_err()
//         {
//             println!("client disconnected");
//             return;
//         }
//         sleep(Duration::from_secs(3)).await;
//     }
// }

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    if let Err(err) = setup::setup(&conn).await {
        warn!(error = %err);
    }
    let app = Router::new().route("/authorize", post(authorize));

    let server_addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("failed to parse bind address");

    info!(message = "server listening", address = %server_addr);
    axum::Server::bind(&server_addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to server");
}
