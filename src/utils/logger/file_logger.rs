use std::{
    fs::{self, File},
    io::Error,
    path::Path,
};

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

    fn write_to_file(&self, log: &str) {}
    fn open_file(&self) -> File {
        todo!()
    }
}

impl Logger for FileLogger {
    fn info(&self, log: &str) {
        todo!()
    }

    fn warning(&self, log: &str) {
        todo!()
    }

    fn error(&self, log: &str) {
        todo!()
    }
}
