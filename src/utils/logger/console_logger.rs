use colored::Colorize;
use sqlx::types::chrono;

use super::Logger;

pub struct ConsoleLogger {}

impl ConsoleLogger {
    pub fn new() -> ConsoleLogger {
        ConsoleLogger {}
    }
}

impl Logger for ConsoleLogger {
    fn info(&self, log: &str) {
        println!("{} {}: {}", "info".green(), chrono::Utc::now(), log)
    }

    fn warning(&self, log: &str) {
        println!("{} {}: {}", "warning".yellow(), chrono::Utc::now(), log)
    }

    fn error(&self, log: &str) {
        println!("{} {}: {}", "error".red(), chrono::Utc::now(), log)
    }
}
