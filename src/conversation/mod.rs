use crate::{action::Action, message::Message};

use completion::completion;
mod completion;
mod error_prompts;


pub struct Conversation {
    history: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Self {
        Conversation {
            history: Vec::new(),
        }
    }

    // TODO: should we have a different error type? does this even throw an error?
    pub async fn process_turn(
        &mut self,
        user_input: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.history.push(Message::User(user_input));

        // loop until a reply is sent
        loop {
            let action = match completion(&self.history).await {
                Ok(action) => action,
                Err(e) => {
                    self.history.push(Message::Observation(format!("{}", e)));
                    continue
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
                            observation = format!(
                                "[OBSERVATION] Tool '{}' executed successfully. Result: {}",
                                tool_name, res
                            );
                        }
                        Err(e) => {
                            observation =
                                format!("[OBSERVATION] Tool '{}' failed. Error: {}", tool_name, e);
                        }
                    }

                    self.history.push(Message::Assistant(
                        serde_json::to_string(&tool_call).unwrap(),
                    ));
                    self.history.push(Message::Observation(observation));
                }
            }
        }
    }
}
