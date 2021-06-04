use std::fmt;
use core::fmt::Formatter;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct StudentError {
    details: String
}

impl StudentError {
    pub fn new(msg: String) -> StudentError {
        StudentError {
            details: msg
        }
    }
}

impl fmt::Display for StudentError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for StudentError {
    fn description(&self) -> &str {
        &self.details
    }
}