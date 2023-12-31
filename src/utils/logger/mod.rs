pub mod console_logger;
pub mod file_logger;

pub trait Logger {
    fn info(&self, log: &str);
    fn warning(&self, log: &str);
    fn error(&self, log: &str);
}
