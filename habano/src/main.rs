use std::time::Instant;

mod error;
use error::{FileErrorPackage, HabanoError};

mod lexer;
use lexer::lex;
use vm::execute;

mod compiler;
mod vm;

fn execute_app() -> Result<(), HabanoError> {
    if std::env::args().len() != 2 {
        return Err(HabanoError::ArgumentError);
    }

    let filename = std::env::args().nth(1).unwrap();
    println!("Reading file: {}", filename);

    let contents = std::fs::read_to_string(filename.clone())
        .map_err(|_| HabanoError::CannotOpenFile(FileErrorPackage { filename }))?;

    let tokens = lex(&contents);
    let irs = compiler::compile(&tokens);

    for (index, ir) in irs.iter().enumerate() {
        println!("{} {:?}", index, ir);
    }

    execute(irs).map_err(|op_err| HabanoError::VmError(op_err))?;

    Ok(())
}

fn main() {
    let start = Instant::now();

    let result = match execute_app() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    };

    let duration = start.elapsed();
    println!("\nExecution time: {:?}", duration);

    result
}
