use std::{fs, process::Command};

use serde::{Deserialize, Serialize};

use crate::bootstrap::logger;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReadArgs {
    path: String,
}

#[cfg(test)]
impl ReadArgs {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

pub fn read(path: &str) -> Result<String, String> {
    logger().info(format!("Attempting to read from {}", path));
    match fs::read_to_string(path) {
        Ok(content) => {
            logger().info(format!(
                "Successfully read content from {}: {}",
                path, content
            ));
            Ok(content)
        }
        Err(e) => {
            logger().error(format!("Error reading from {}: {}", path, e));
            Err(format!("File read failed for {}: {}", path, e))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WriteArgs {
    path: String,
    content: String,
}

pub fn write(path: &str, content: &str) -> Result<String, String> {
    logger().info(format!("Attempting to write to {}", path));
    match fs::write(path, content) {
        Ok(_) => {
            logger().info(format!("Successfully wrote to {}: {}", path, content));
            Ok("".to_string()) // TODO: surely we can improve on this return
        }
        Err(e) => {
            logger().error(format!("Error writing to {}: {}", path, e));
            Err(format!("File write failed for {}: {}", path, e))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ExecArgs {
    args: Vec<String>,
}

#[cfg(test)]
impl ExecArgs {
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }
}

pub fn exec(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        logger().error("Unable to exec with no args".to_string());
        return Err("Command arguments list is empty.".to_string());
    }

    logger().info(format!("Attempting to exec: {:?}", args));
    let program = &args[0];
    let command_args: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();

    let output = Command::new(program)
        .args(&command_args)
        .output()
        .inspect_err(|e| logger().error(format!("Failed to execute command {}: {}", program, e)))
        .map_err(|e| format!("Failed to execute command '{}': {}", program, e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    logger().info(format!(
        "Command executed with status code {:?}",
        output.status.code()
    ));
    logger().info(format!("Stdout: {:?}", stdout));
    logger().info(format!("Stderr: {:?}", stderr));

    if output.status.success() {
        Ok(format!("Stdout: {}\nStderr: {}", stdout, stderr))
    } else {
        let error_message = format!(
            "Command failed with exit code {}.\nStdout: {}\nStderr: {}",
            output.status, stdout, stderr
        );

        logger().error(error_message.clone());
        Err(error_message)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "tool", rename_all = "snake_case")]
pub enum ToolCall {
    Read(ReadArgs),
    Exec(ExecArgs),
    Write(WriteArgs),
}

impl ToolCall {
    pub fn name(&self) -> String {
        let s = match self {
            ToolCall::Read(_) => "read",
            ToolCall::Exec(_) => "exec",
            ToolCall::Write(_) => "write",
        };

        s.to_string()
    }

    pub fn call(&self) -> Result<String, String> {
        match self {
            ToolCall::Read(args) => read(&args.path),
            ToolCall::Exec(args) => exec(&args.args),
            ToolCall::Write(args) => write(&args.path, &args.content),
        }
    }
}
