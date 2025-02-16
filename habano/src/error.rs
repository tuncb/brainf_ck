use std::error::Error;
use std::fmt;

use crate::vm::VmErrorPackage;

#[derive(Debug)]
pub struct FileErrorPackage {
    pub filename: String,
}

#[derive(Debug)]
pub enum HabanoError {
    ArgumentError,
    CannotOpenFile(FileErrorPackage),
    VmError(VmErrorPackage),
}

impl fmt::Display for HabanoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HabanoError::ArgumentError => write!(f, "Usage: habano <filename>"),
            HabanoError::CannotOpenFile(e) => write!(f, "Cannot open file: {}", e.filename),
            HabanoError::VmError(e) => write!(
                f,
                "VM error: {:?} at position {} with IR {:?}",
                e.err, e.position, e.ir
            ),
        }
    }
}

impl Error for HabanoError {}
