#[derive(Debug)]
pub enum LexingError {
    IllegalOperation,
    IllegalOperand,
    IllegalLabel,
}

#[derive(Debug)]
pub enum ParsingError {
    StackUnderflow,
    DivisionByZero,
    InvalidInstructionPointer,
    IllegalOperand,
}
