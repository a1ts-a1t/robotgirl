use serde::{Serialize, Deserialize};

use crate::tools::ToolCall;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "intent", rename_all = "snake_case")]
pub enum Action {
    Reply { content: String }, 
    ToolCall(ToolCall),
}
