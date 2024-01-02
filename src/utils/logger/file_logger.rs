use std::{
    fs::File,
    io::{Error, Write},
};

use chrono::Utc;

use super::Logger;

pub struct FileLogger {
    directory: String,
}

impl FileLogger {
    pub fn new(path: &str) -> Result<Self, Error> {
        Ok(FileLogger {
            directory: path.to_string(),
        })
    }

    fn write_to_file(&self, log: &str) {
        self.open_file().write_all(log.as_bytes()).unwrap()
    }
    fn open_file(&self) -> File {
        match File::options().append(true).open(format!(
            "{}/{}",
            self.directory.clone(),
            Utc::now().date_naive()
        )) {
            Ok(file) => file,
            Err(_) => File::create(format!("{}/{}", self.directory, Utc::now())).unwrap(),
        }
    }
}

impl Logger for FileLogger {
    fn info(&self, log: &str) {
        self.write_to_file(log)
    }

    fn warning(&self, log: &str) {
        self.write_to_file(log)
    }

    fn error(&self, log: &str) {
        self.write_to_file(log)
    }
}
