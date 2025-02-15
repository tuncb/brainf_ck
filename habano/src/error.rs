use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct FileErrorPackage {
    pub filename: String,
}

#[derive(Debug)]
pub enum HabanoError {
    ArgumentError,
    CannotOpenFile(FileErrorPackage),
}

impl fmt::Display for HabanoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HabanoError::ArgumentError => write!(f, "Usage: habano <filename>"),
            HabanoError::CannotOpenFile(e) => write!(f, "Cannot open file: {}", e.filename),
        }
    }
}

impl Error for HabanoError {}
