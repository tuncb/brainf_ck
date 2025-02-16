use std::error::Error;
use std::fmt;

use crate::compiler::CompilerError;
use crate::vm::VmError;

#[derive(Debug)]
pub struct FileErrorPackage {
    pub filename: String,
}

#[derive(Debug)]
pub enum HabanoError {
    ArgumentError,
    CannotOpenFile(FileErrorPackage),
    HabanoCompilerError(Vec<CompilerError>),
    HabanoVmError(VmError),
}

impl fmt::Display for HabanoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HabanoError::ArgumentError => write!(f, "Usage: habano <filename>"),
            HabanoError::CannotOpenFile(e) => write!(f, "Cannot open file: {}", e.filename),
            HabanoError::HabanoVmError(e) => write!(
                f,
                "VM error: {:?} at position {} with IR {:?}",
                e.err, e.position, e.ir
            ),
            HabanoError::HabanoCompilerError(errors) => {
                write!(f, "Found {} compiler errors:\n", errors.len())?;

                for error in errors {
                    write!(
                        f,
                        "Compiler error: {:?} at line: {}, column: {}\n",
                        error.err, error.position.line, error.position.column
                    )?;
                }
                Ok(())
            }
        }
    }
}

impl Error for HabanoError {}
