use std::time::Instant;

mod error;
use error::{FileErrorPackage, HabanoError};

#[derive(Debug)]
enum BfCommands {
    Add,
    Dec,
    IncrementPointer,
    DecrementPointer,
    LoopStart,
    LoopEnd,
    Input,
    Output,
}

fn parse_commands(contents: &str) -> Vec<BfCommands> {
    let mut commands = Vec::new();

    for c in contents.chars() {
        match c {
            '+' => commands.push(BfCommands::Add),
            '.' => commands.push(BfCommands::Output),
            ',' => commands.push(BfCommands::Input),
            '-' => commands.push(BfCommands::Dec),
            '<' => commands.push(BfCommands::DecrementPointer),
            '>' => commands.push(BfCommands::IncrementPointer),
            '[' => commands.push(BfCommands::LoopStart),
            ']' => commands.push(BfCommands::LoopEnd),
            _ => (),
        }
    }

    commands
}

fn run_commands(commands: Vec<BfCommands>) {
    let mut memory = [0i8; 30000];
    let mut pointer = 0;
    let mut program_counter = 0;

    while program_counter < commands.len() {
        match commands[program_counter] {
            BfCommands::Add => memory[pointer] = memory[pointer].wrapping_add(1),
            BfCommands::Dec => memory[pointer] = memory[pointer].wrapping_sub(1),
            BfCommands::IncrementPointer => pointer = pointer.wrapping_add(1),
            BfCommands::DecrementPointer => pointer = pointer.wrapping_sub(1),
            BfCommands::Output => {
                let byte = memory[pointer] as u8;
                print!("{}", byte as char);
            }
            BfCommands::LoopStart => {
                if memory[pointer] == 0 {
                    // Find matching ]
                    let mut depth = 1;
                    while depth > 0 {
                        program_counter += 1;
                        match commands[program_counter] {
                            BfCommands::LoopStart => depth += 1,
                            BfCommands::LoopEnd => depth -= 1,
                            _ => (),
                        }
                    }
                }
            }
            BfCommands::LoopEnd => {
                if memory[pointer] != 0 {
                    // Find matching [
                    let mut depth = 1;
                    while depth > 0 {
                        program_counter -= 1;
                        match commands[program_counter] {
                            BfCommands::LoopStart => depth -= 1,
                            BfCommands::LoopEnd => depth += 1,
                            _ => (),
                        }
                    }
                }
            }
            BfCommands::Input => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                // Parse the input as a number
                memory[pointer] = input.trim().parse::<i8>().unwrap_or(0); // Default to 0 if parsing fails
            }
        }
        program_counter += 1;
    }
}

fn execute() -> Result<(), HabanoError> {
    if std::env::args().len() != 2 {
        return Err(HabanoError::ArgumentError);
    }

    let filename = std::env::args().nth(1).unwrap();
    println!("Reading file: {}", filename);

    let contents = std::fs::read_to_string(filename.clone())
        .map_err(|_| HabanoError::CannotOpenFile(FileErrorPackage { filename }))?;

    let commands = parse_commands(&contents);
    run_commands(commands);

    Ok(())
}

fn main() {
    let start = Instant::now();

    let result = match execute() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    };

    let duration = start.elapsed();
    println!("\nExecution time: {:?}", duration);

    result
}
