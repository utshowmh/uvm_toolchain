use crate::global::Float;

#[derive(Debug)]
pub enum InstructionType {
    Push,
    Pop,

    Duplicate,
    Swap,

    Jump,
    JumpIf,

    Equal,
    GreaterEqual,
    Not,

    Add,
    Subtract,
    Multiply,
    Divide,

    Dump,
    Output,
    Outputf,

    Halt,
}

#[derive(Debug)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub operand: Option<Float>,
}

impl Instruction {
    pub fn new(instruction_type: InstructionType, operand: Option<Float>) -> Self {
        Self {
            instruction_type,
            operand,
        }
    }
}
