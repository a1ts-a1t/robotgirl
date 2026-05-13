use std::env;
use std::path::Path;
use std::sync::Mutex;
use std::{error::Error, fs, io};

use lazy_static::lazy_static;
use minijinja::Error as JinjaError;
use minijinja::{Environment, ErrorKind};
use serde::Serialize;

pub struct Config {
    jinja_env: Environment<'static>,
    llama_server_url: String,
    model: String, 
    temperature: f64,
}

impl Config {
    fn new() -> Self {
        let mut jinja_env = Environment::new();
        jinja_env.set_loader(|name| {
            let path = Path::new("./prompts").join(format!("{}.jinja", name));
            match fs::read_to_string(path) {
                Ok(result) => Ok(Some(result)),
                Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
                Err(err) => Err(JinjaError::new(
                    ErrorKind::InvalidOperation,
                    "could not read template",
                )
                .with_source(err)),
            }
        });

        let llama_server_url = env::var("LLAMA_SERVER_URL").unwrap_or("http://localhost:8080".to_string());
        let model = env::var("MODEL").unwrap_or("gemma-4-e4b".to_string());
        let temperature = env::var("TEMPERATURE")
            .ok()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.1);
        Self { jinja_env, llama_server_url, model, temperature }
    }

    pub fn load_prompt<S: Serialize>(
        &mut self,
        name: &str,
        ctx: S,
    ) -> Result<String, Box<dyn Error>> {
        let template = match self.jinja_env.get_template(name) {
            Ok(t) => t,
            Err(e) => return Err(Box::new(e)),
        };

        match template.render(ctx) {
            Ok(s) => Ok(s),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn llama_server_url(&self) -> String {
        self.llama_server_url.clone()
    }

    pub fn model(&self) -> String {
        self.model.clone()
    }

    pub fn temperature(&self) -> f64 {
        self.temperature.clone()
    }
}

lazy_static! {
    static ref GLOBAL_CONFIG: Mutex<Config> = Mutex::new(Config::new());
}

pub fn config() -> std::sync::MutexGuard<'static, Config> {
    GLOBAL_CONFIG.lock().unwrap()
}
