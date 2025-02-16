#[derive(Debug, Clone)]
pub enum VmIrOperands {
    Add,
    MoveMemoryPointer,
    ReadFromConsole,
    WriteToConsole,
    GotoIfZero,
    GotoIfNonZero,
}

#[derive(Debug, Clone)]
pub struct VmIr {
    pub operand: VmIrOperands,
    pub operand_value: i64,
}

#[derive(Debug)]
pub enum VmErrorType {
    Overflow,
    AccessViolation,
}

#[derive(Debug)]
pub struct VmError {
    pub err: VmErrorType,
    pub ir: VmIr,
    pub position: usize,
}

struct VmState {
    memory: [i8; 30000],
    pointer: usize,
    program_counter: usize,
}

pub fn execute(commands: Vec<VmIr>) -> Result<(), VmError> {
    let mut vm = VmState {
        memory: [0i8; 30000],
        pointer: 0,
        program_counter: 0,
    };

    while vm.program_counter < commands.len() {
        let ir = &commands[vm.program_counter];

        match ir.operand {
            VmIrOperands::Add => {
                let value = i8::try_from(ir.operand_value).map_err(|_| VmError {
                    err: VmErrorType::Overflow,
                    ir: ir.clone(),
                    position: vm.program_counter,
                })?;

                vm.memory[vm.pointer] = vm.memory[vm.pointer].wrapping_add(value);
            }
            VmIrOperands::MoveMemoryPointer => {
                if ir.operand_value < 0 || ir.operand_value >= 30000 {
                    return Err(VmError {
                        err: VmErrorType::AccessViolation,
                        ir: ir.clone(),
                        position: vm.program_counter,
                    });
                };

                vm.pointer = ir.operand_value as usize;
            }
            VmIrOperands::ReadFromConsole => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                // Parse the input as a number
                vm.memory[vm.pointer] = input.trim().parse::<i8>().unwrap_or(0);
                // Default to 0 if parsing fails
            }
            VmIrOperands::WriteToConsole => {
                let byte = vm.memory[vm.pointer] as u8;
                print!("{}", byte as char);
            }
            VmIrOperands::GotoIfZero => {
                if (ir.operand_value < 0) || ((ir.operand_value as usize) >= commands.len()) {
                    return Err(VmError {
                        err: VmErrorType::AccessViolation,
                        ir: ir.clone(),
                        position: vm.program_counter,
                    });
                }

                if vm.memory[vm.pointer] == 0 {
                    vm.program_counter = ir.operand_value as usize;
                }
            }
            VmIrOperands::GotoIfNonZero => {
                if (ir.operand_value < 0) || ((ir.operand_value as usize) >= commands.len()) {
                    return Err(VmError {
                        err: VmErrorType::AccessViolation,
                        ir: ir.clone(),
                        position: vm.program_counter,
                    });
                }
                if vm.memory[vm.pointer] != 0 {
                    vm.program_counter = ir.operand_value as usize;
                }
            }
        }
        vm.program_counter += 1;
    }

    Ok(())
}
