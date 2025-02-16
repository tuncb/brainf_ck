use crate::{
    lexer::{BfTokenType, TokenPosition, Tokens},
    vm::{VmIr, VmIrOperands},
};

#[derive(Debug)]
pub enum CompilerErrType {
    UnmatchedLoopStart,
    UnmatchedLoopEnd,
}

#[derive(Debug)]
pub struct CompilerError {
    pub err: CompilerErrType,
    pub position: TokenPosition,
}

pub fn compile(tokens: &Tokens) -> Result<Vec<VmIr>, Vec<CompilerError>> {
    let mut ir = Vec::new();
    let mut stack = Vec::new();
    let mut errors: Vec<CompilerError> = Vec::new();

    let mut pointer = 0;

    for (index, (token, position)) in tokens
        .token_types
        .iter()
        .zip(tokens.positions.iter())
        .enumerate()
    {
        match token {
            BfTokenType::Add => ir.push(VmIr {
                operand: VmIrOperands::Add,
                operand_value: 1,
            }),
            BfTokenType::Dec => ir.push(VmIr {
                operand: VmIrOperands::Add,
                operand_value: -1,
            }),
            BfTokenType::IncrementPointer => {
                pointer += 1;
                ir.push(VmIr {
                    operand: VmIrOperands::MoveMemoryPointer,
                    operand_value: pointer,
                });
            }
            BfTokenType::DecrementPointer => {
                pointer -= 1;
                ir.push(VmIr {
                    operand: VmIrOperands::MoveMemoryPointer,
                    operand_value: pointer,
                });
            }
            BfTokenType::Output => ir.push(VmIr {
                operand: VmIrOperands::WriteToConsole,
                operand_value: 0,
            }),
            BfTokenType::Input => ir.push(VmIr {
                operand: VmIrOperands::ReadFromConsole,
                operand_value: 0,
            }),
            BfTokenType::LoopStart => {
                ir.push(VmIr {
                    operand: VmIrOperands::GotoIfZero,
                    operand_value: 0,
                });
                stack.push(index);
            }
            BfTokenType::LoopEnd => {
                let start_opt = stack.pop();
                if start_opt.is_none() {
                    errors.push(CompilerError {
                        err: CompilerErrType::UnmatchedLoopEnd,
                        position: position.clone(),
                    });
                } else {
                    let start = start_opt.unwrap();
                    ir[start].operand_value = (index as i64) + 1;
                    ir.push(VmIr {
                        operand: VmIrOperands::GotoIfNonZero,
                        operand_value: start as i64,
                    });
                }
            }
        }
    }

    if !stack.is_empty() {
        for index in stack {
            errors.push(CompilerError {
                err: CompilerErrType::UnmatchedLoopStart,
                position: tokens.positions[index].clone(),
            });
        }
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(ir)
    }
}
