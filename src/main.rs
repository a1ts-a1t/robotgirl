use std::collections::HashMap;

use axum::{Router, routing::post};

use crate::{api::{handle_chat}, conversation::Conversation};

mod action;
mod tools;
mod message;
mod conversation;
mod api;

#[tokio::main]
async fn main() {
    let session_map: HashMap<String, Conversation> = HashMap::new();
    let sessions_state = std::sync::Arc::new(tokio::sync::Mutex::new(session_map));

    // Define the routes
    let app = Router::new()
        .route("/chat", post(handle_chat))
        .with_state(sessions_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("RobotGirl Server listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

