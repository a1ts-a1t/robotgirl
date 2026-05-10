use std::{fs, process::Command};

use serde::{Deserialize, Serialize};

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
    // TODO: logging
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("File read failed for {}: {}", path, e)),
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WriteArgs {
    path: String,
    content: String,
}

pub fn write(path: &str, content: &str) -> Result<String, String> {
    // TODO: logging
    match fs::write(path, content) {
        Ok(_) => Ok("".to_string()), // TODO: surely we can improve on this return
        Err(e) => Err(format!("File write failed for {}: {}", path, e)),
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
    // TODO: logging
    if args.is_empty() {
        return Err("Command arguments list is empty.".to_string());
    }

    let program = &args[0];
    let command_args: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();

    let output = Command::new(program)
        .args(&command_args) // Command::args accepts a slice of &str
        .output()
        .map_err(|e| format!("Failed to execute command '{}': {}", program, e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(stdout) // TODO: we probably also want to expose stderr in the ok case
    } else {
        Err(format!("Command failed with exit code {}. Stderr: {}", output.status, stderr))
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

