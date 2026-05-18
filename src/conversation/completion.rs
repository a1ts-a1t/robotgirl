use std::{collections::VecDeque};
use std::error::Error;

use reqwest::{Client, Response};
use serde_json::{Value, json};

use crate::conversation::error_prompts::error_prompt;
use crate::{action::Action, message::Message};
use crate::bootstrap::{config, logger};

fn create_body(messages: &[Message]) -> Result<Value, Box<dyn Error>> {
    let base_system_prompt = config()
        .load_prompt("SYSTEM_PROMPT.md", {})
        .map_err(|_| "Failed to load system prompt")?;

    let mut api_messages: VecDeque<Value> = messages
        .iter()
        .map(|message| match message {
            Message::User(content) => json!({ "role": "user", "content": content }),
            Message::Assistant(content) => json!({ "role": "assistant", "content": content }),
            Message::Observation(content) => json!({ "role": "user", "content": content }),
        })
        .collect();

    api_messages.push_front(json!({ "role": "system", "content": base_system_prompt }));

    Ok(json!({
        "model": config().model(), 
        "temperature": config().temperature(),
        "response_format": { "type": "json_object" },
        "messages": api_messages,
    }))
}

async fn fetch_model_response(body: Value) -> Result<Response, Box<dyn Error>> {
    let client = Client::new();
    let chat_url = format!("{}/v1/chat/completions", config().llama_server_url());

    let response = client
        .post(&chat_url)
        .json(&body)
        .send()
        .await
        .and_then(Response::error_for_status)
        .inspect_err(|e| logger().error(format!("Error while fetching model response: {}", e)))
        .map_err(|e| error_prompt("network", e))?;

    Ok(response)
}

/**
 * The output of this should be directly convertable deserializable into an Action.
 */
async fn sanitize_response(response: Response) -> Result<String, Box<dyn Error>> {
    let json: Value = response.json()
        .await
        .inspect_err(|e| logger().error(format!("Error while sanitizing model repsonse: {}", e)))
        .map_err(|e| error_prompt("format", e))?;

    let content_value = json["choices"][0]["message"]["content"].clone();

    let json_string = content_value
        .as_str()
        .unwrap_or(""); // will be handled as a formatting error later

    Ok(json_string.trim_matches('"').to_string())
}

pub async fn completion(messages: &[Message]) -> Result<Action, Box<dyn Error>> {
    let body = create_body(messages)?;
    logger().info(format!("Requesting model response with body: {}", serde_json::to_string_pretty(&body).unwrap()));

    let response = fetch_model_response(body).await?;
    logger().info(format!("Model response: {:?}", response));

    let sanitized_response = sanitize_response(response).await?;
    logger().info(format!("Sanitized model response: {}", sanitized_response));

    let action = serde_json::from_str::<Action>(&sanitized_response)
        .inspect_err(|e| logger().error(format!("Error while deserializing model response: {}", e)))
        .map_err(|e| error_prompt("format", e))?;

    Ok(action)
}
