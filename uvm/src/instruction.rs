use crate::global::Integer;

#[derive(Debug)]
pub enum InstructionType {
    Push,
    Pop,
    Duplicate,
    Jump,
    JumpIf,

    Equal,
    Add,
    Subtract,
    Multiply,
    Divide,
    Print,
    Halt,
}

#[derive(Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub operand: Option<Integer>,
}

impl Instruction {
    pub fn new(instruction_type: InstructionType, operand: Option<Integer>) -> Self {
        Self {
            instruction_type,
            operand,
        }
    }
}
