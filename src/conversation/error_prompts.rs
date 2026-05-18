use std::error::Error;

use minijinja::context;

use crate::bootstrap::config;

pub fn error_prompt<E: Error>(error_type: &str, error: E) -> String {
    let context = context! {
        error_type => error_type,
        error_str => format!("{}", error)
    };

    let fallback_message = format!("Failed to load {} error prompt", error_type);

    config()
        .load_prompt("COMPLETION_ERROR.md", context)
        .unwrap_or(fallback_message)
}

