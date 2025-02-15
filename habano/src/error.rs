use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct FileErrorPackage {
    pub filename: String,
}

#[derive(Debug)]
pub struct LexerErrorPackage {
    pub character: char,
    pub line: usize,
    pub position: usize,
}

#[derive(Debug)]
pub enum HabanoError {
    ArgumentError,
    CannotOpenFile(FileErrorPackage),
    InvalidCharacter(LexerErrorPackage),
}

impl fmt::Display for HabanoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HabanoError::ArgumentError => write!(f, "Usage: habano <filename>"),
            HabanoError::CannotOpenFile(e) => write!(f, "Cannot open file: {}", e.filename),
            HabanoError::InvalidCharacter(e) => write!(
                f,
                "Invalid character '{}' at line {}, position {}",
                e.character, e.line, e.position
            ),
        }
    }
}

impl Error for HabanoError {}
