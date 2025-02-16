use crate::{
    lexer::BfToken,
    vm::{VmIr, VmIrOperands},
};

pub fn compile(tokens: &Vec<BfToken>) -> Vec<VmIr> {
    let mut ir = Vec::new();
    let mut stack = Vec::new();

    let mut pointer = 0;

    for (index, token) in tokens.iter().enumerate() {
        match token {
            BfToken::Add => ir.push(VmIr {
                operand: VmIrOperands::Add,
                operand_value: 1,
            }),
            BfToken::Dec => ir.push(VmIr {
                operand: VmIrOperands::Add,
                operand_value: -1,
            }),
            BfToken::IncrementPointer => {
                pointer += 1;
                ir.push(VmIr {
                    operand: VmIrOperands::MoveMemoryPointer,
                    operand_value: pointer,
                });
            }
            BfToken::DecrementPointer => {
                pointer -= 1;
                ir.push(VmIr {
                    operand: VmIrOperands::MoveMemoryPointer,
                    operand_value: pointer,
                });
            }
            BfToken::Output => ir.push(VmIr {
                operand: VmIrOperands::WriteToConsole,
                operand_value: 0,
            }),
            BfToken::Input => ir.push(VmIr {
                operand: VmIrOperands::ReadFromConsole,
                operand_value: 0,
            }),
            BfToken::LoopStart => {
                ir.push(VmIr {
                    operand: VmIrOperands::GotoIfZero,
                    operand_value: 0,
                });
                stack.push(index);
            }
            BfToken::LoopEnd => {
                let start = stack.pop().unwrap();
                ir[start].operand_value = (index as i64) + 1;
                ir.push(VmIr {
                    operand: VmIrOperands::GotoIfNonZero,
                    operand_value: start as i64,
                });
            }
        }
    }

    ir
}
