use std::{env, fs};

use reqwest::Client;
use serde_json::{Value, json};

use crate::{action::Action, message::Message};

// TODO: this should be in a bootstrap or config or something
const PROMPT_FILE: &str = "SYSTEM_PROMPT.md";
fn load_system_prompt() -> Result<String, std::io::Error> {
    // TODO: logging
    println!("🔍 Loading system prompt from {}...", PROMPT_FILE);
    fs::read_to_string(PROMPT_FILE)
}

pub async fn completion(messages: &[Message]) -> Result<Action, Box<dyn std::error::Error>> {
    let base_system_prompt = match load_system_prompt() {
        Ok(p) => p,
        Err(e) => {
            return Err(format!("Failed to load system prompt file '{}': {}", PROMPT_FILE, e).into());
        }
    };

    let api_messages: Vec<Value> = messages.iter().map(|message| {
        match message {
            Message::User(content) => json!({ "role": "user", "content": content }),
            Message::Assistant(content) => json!({ "role": "assistant", "content": content }),
            Message::Observation(content) => json!({ "role": "user", "content": content }),
        }
    }).collect();

    let body = json!({
        // TODO: this should be in a config
        "temperature": 0.1,
        "response_format": { "type": "json_object" },
        "messages": [
            { "role": "system", "content": base_system_prompt },
            api_messages,
        ],
    });

    // TODO: this should be in a config
    let api_url = env::var("LLAMA_SERVER_URL")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    
    let client = Client::new();
    let chat_url = format!("{}/v1/chat/completions", api_url);
    
    let response = client.post(&chat_url)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;

    let json_response: Value = response.json().await?;
    let content_value = json_response["choices"][0]["message"]["content"].clone();

    match serde_json::from_value::<Action>(content_value) {
        Ok(action) => Ok(action),
        Err(e) => Err(Box::new(e)),
    }

}

pub struct Conversation {
    history: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Self {
        Conversation { history: Vec::new() }
    }

    // TODO: should we have a different error type? does this even throw an error?
    pub async fn process_turn(&mut self, user_input: String) -> Result<String, Box<dyn std::error::Error>> {
        self.history.push(Message::User(user_input));

        // loop until a reply is sent
        loop {
            let action = match completion(&self.history).await {
                Ok(action) => action,
                Err(e) => {
                    let error_prompt = format!("Parsing Error: {}", e);
                    eprintln!("{}", error_prompt);
                    self.history.push(Message::Assistant(error_prompt)); // Record the error
                    continue;
                }
            };

            match action {
                Action::Reply { content } => {
                    self.history.push(Message::Assistant(content.clone()));
                    return Ok(content);
                }
                
                Action::ToolCall(tool_call) => {
                    // TODO: logging
                    let tool_result = tool_call.call();
                    let tool_name = tool_call.name();
                    
                    let observation: String;
                    match tool_result {
                        Ok(res) => {
                            // TODO: standardize formatting
                            observation = format!("[OBSERVATION] Tool '{}' executed successfully. Result: {}", tool_name, res);
                        },
                        Err(e) => {
                            observation = format!("[OBSERVATION] Tool '{}' failed. Error: {}", tool_name, e);
                        }
                    }

                    self.history.push(Message::Assistant(serde_json::to_string(&tool_call).unwrap()));
                    self.history.push(Message::Observation(observation));
                }
            }
        }
    }
}
