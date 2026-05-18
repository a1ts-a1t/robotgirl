use std::{collections::HashMap, env};

use axum::{Router, routing::post};
use tower_http::services::ServeDir;

use crate::{api::handle_chat, bootstrap::logger, conversation::Conversation};

mod action;
mod api;
mod conversation;
mod message;
mod tools;
mod bootstrap;

#[tokio::main]
async fn main() {
    let session_map: HashMap<String, Conversation> = HashMap::new();
    let sessions_state = std::sync::Arc::new(tokio::sync::Mutex::new(session_map));

    // Define the routes
    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/api/chat", post(handle_chat))
        .with_state(sessions_state);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    let address = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(address.clone())
        .await
        .unwrap();
    logger().info(format!("RobotGirl Server listening on {}", address));

    axum::serve(listener, app).await.unwrap();
}
