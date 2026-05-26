use std::sync::Mutex;

use lazy_static::lazy_static;
use textwrap::{Options, termwidth, wrap};

#[derive(Clone)]
pub struct Logger {}

impl Logger {
    fn new() -> Self {
        Self {}
    }

    pub fn info(&self, message: String) {
        let options = Options::new(termwidth())
            .initial_indent("[INFO]  ")
            .subsequent_indent("        ");
        let lines = wrap(&message, options);
        println!("{}", lines.join("\n"));
    }

    #[expect(unused)]
    pub fn warn(&self, message: String) {
        let options = Options::new(termwidth())
            .initial_indent("[WARN]  ")
            .subsequent_indent("        ");
        let lines = wrap(&message, options);
        println!("{}", lines.join("\n"));
    }

    pub fn error(&self, message: String) {
        let options = Options::new(termwidth())
            .initial_indent("[ERROR] ")
            .subsequent_indent("        ");
        let lines = wrap(&message, options);
        println!("{}", lines.join("\n"));
    }
}

lazy_static! {
    static ref GLOBAL_LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

pub fn logger() -> Logger {
    GLOBAL_LOGGER.lock().unwrap().clone()
}
