use serde::{Deserialize, Serialize};

use crate::tools::ToolCall;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "intent", rename_all = "snake_case")]
pub enum Action {
    Reply { content: String },
    ToolCall(ToolCall),
}

#[cfg(test)]
mod test {
    use crate::tools::{ExecArgs, ReadArgs};

    use super::*;

    #[test]
    fn serialize_exec() {
        let exec_args = ExecArgs::new(vec!["ls".to_string()]);
        let tool_call = ToolCall::Exec(exec_args);
        let actual = Action::ToolCall(tool_call);

        let expected = serde_json::from_str::<Action>(
            "{ \"intent\": \"tool_call\", \"tool\": \"exec\", \"args\": [\"ls\"]}",
        )
        .unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn serialize_read() {
        let read_args = ReadArgs::new("file.txt".to_string());
        let tool_call = ToolCall::Read(read_args);
        let actual = Action::ToolCall(tool_call);

        let expected = serde_json::from_str::<Action>(
            "{ \"intent\": \"tool_call\", \"tool\": \"read\", \"path\": \"file.txt\"}",
        )
        .unwrap();

        assert_eq!(expected, actual);
    }
}
