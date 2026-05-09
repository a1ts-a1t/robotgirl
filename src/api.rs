use std::collections::HashMap;

use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::conversation::Conversation;

// The incoming request structure from the client
#[derive(Deserialize)]
pub struct ChatRequest {
    session_id: String,
    user_message: String,
}

// The response structure returned to the client
#[derive(Serialize)]
pub struct ChatResponse {
    response: String,
}

pub async fn handle_chat(
    State(sessions): State<std::sync::Arc<tokio::sync::Mutex<HashMap<String, Conversation>>>>,
    Json(request): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, String)> {
    
    let mut sessions_lock = sessions.lock().await;

    // Get or create the agent for the specific session
    let conversation = sessions_lock
        .entry(request.session_id.clone())
        .or_insert_with(Conversation::new);
    
    // Process the conversation turn
    match conversation.process_turn(request.user_message.clone()).await {
        Ok(final_response) => {
            Ok(Json(ChatResponse { response: final_response }))
        }
        Err(e) => {
            // TODO: logging
            eprintln!("Agent execution failed: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Agent execution error: {}", e)))
        }
    }
}

