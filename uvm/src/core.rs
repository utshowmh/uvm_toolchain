use std::{fs::read_to_string, process::exit};

use crate::{
    error::{LexingError, ParsingError},
    global::{Float, Integer},
    instruction::{Instruction, InstructionType},
    label::{Label, LabelTable},
};

pub struct UVM {
    stack: Vec<Float>,
    program: Vec<Instruction>,
    instruction_pointer: usize,
    label_table: LabelTable,
    halt: bool,
}

impl UVM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            program: Vec::new(),
            instruction_pointer: 0,
            label_table: LabelTable::new(),
            halt: false,
        }
    }

    pub fn emulate(&mut self, filepath: &str, limit: usize) {
        if let Some(err) = self.load_program_from_file(filepath) {
            eprintln!("LexingError: {:#?}", err);
            std::process::exit(1);
        };
        for _ in 0..limit {
            if self.halt {
                break;
            }
            if let Some(err) = self.execute_instruction() {
                eprintln!("ParsingError: {:#?}", err);
                exit(1);
            };
        }
    }

    pub fn run(&mut self, filepath: &str) {
        if let Some(err) = self.load_program_from_file(filepath) {
            eprintln!("LexingError: {:#?}", err);
            std::process::exit(1);
        };
        while !self.halt {
            if let Some(err) = self.execute_instruction() {
                eprintln!("ParsingError: {:#?}", err);
                exit(1);
            };
        }
    }

    fn load_program_from_file(&mut self, filepath: &str) -> Option<LexingError> {
        let source = read_to_string(filepath).unwrap_or_else(|err| {
            eprintln!("ERROR: {}", err);
            exit(1);
        });
        let instructions = source.trim().split("\n");
        let mut instruction_index = 0;

        for instruction in instructions {
            let instruction: Vec<&str> = instruction.trim().split(" ").collect();
            let instruction_len = instruction.len();

            if instruction_len > 1 && instruction[0].starts_with(";") {
                continue;
            }

            match instruction_len {
                1 => {
                    let operation = instruction[0].trim();

                    if operation.is_empty() {
                        continue;
                    }

                    match operation {
                        "pop" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Pop, None));
                        }

                        "eql" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Equal, None));
                        }

                        "geql" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::GreaterEqual, None));
                        }

                        "not" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Not, None));
                        }

                        "add" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Add, None));
                        }

                        "sub" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Subtract, None));
                        }

                        "mul" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Multiply, None));
                        }

                        "div" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Divide, None));
                        }

                        "out" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Output, None));
                        }

                        "outf" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Outputf, None));
                        }

                        "dmp" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Dump, None));
                        }

                        "hlt" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Halt, None));
                        }

                        _ => {
                            if operation.starts_with(".") && operation.ends_with(":") {
                                let label_name = instruction[0]
                                    .strip_prefix(".")
                                    .unwrap()
                                    .strip_suffix(":")
                                    .unwrap()
                                    .to_string();
                                self.label_table
                                    .push(Label::new(label_name, instruction_index));
                            } else {
                                return Some(LexingError::IllegalLabel);
                            }
                        }
                    }
                }

                2 => {
                    let operation = instruction[0].trim();
                    let operand = instruction[1].trim();

                    let operand: Float = match operand.parse() {
                        Ok(operand) => operand,
                        Err(_) => {
                            if let Some(operand) = self.label_table.find(operand) {
                                operand as Float
                            } else {
                                return Some(LexingError::IllegalOperand);
                            }
                        }
                    };

                    match operation {
                        "push" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Push, Some(operand)));
                        }

                        "dup" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Duplicate, Some(operand)));
                        }

                        "swp" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Swap, Some(operand)));
                        }

                        "jmp" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::Jump, Some(operand)));
                        }

                        "jmpif" => {
                            instruction_index += 1;
                            self.program
                                .push(Instruction::new(InstructionType::JumpIf, Some(operand)));
                        }

                        _ => {
                            return Some(LexingError::IllegalOperation);
                        }
                    }
                }

                _ => return Some(LexingError::IllegalOperand),
            }
        }
        None
    }

    fn execute_instruction(&mut self) -> Option<ParsingError> {
        if self.instruction_pointer >= self.program.len() {
            return Some(ParsingError::InvalidInstructionPointer);
        }
        let instruction = &self.program[self.instruction_pointer];

        match instruction.instruction_type {
            InstructionType::Push => {
                self.instruction_pointer += 1;

                if let Some(operand) = instruction.operand {
                    self.stack.push(operand);
                } else {
                    return Some(ParsingError::IllegalOperand);
                }
            }

            InstructionType::Pop => {
                self.instruction_pointer += 1;

                if self.stack.len() < 1 {
                    return Some(ParsingError::StackUnderflow);
                }

                self.stack.pop();
            }

            InstructionType::Duplicate => {
                self.instruction_pointer += 1;

                if let Some(instruction_pointer) = instruction.operand {
                    let stack_length = self.stack.len() as Float;
                    if stack_length - instruction_pointer < 1. {
                        return Some(ParsingError::StackUnderflow);
                    }
                    if instruction_pointer < 0. {
                        return Some(ParsingError::IllegalOperand);
                    } else {
                        // it's performing a relative jump; jumping <operand> up.
                        self.stack
                            .push(self.stack[(stack_length - 1. - instruction_pointer) as usize]);
                    }
                }
            }

            InstructionType::Swap => {
                self.instruction_pointer += 1;

                if let Some(instruction_pointer) = instruction.operand {
                    let stack_length = self.stack.len() as Float;
                    if stack_length - instruction_pointer < 1. {
                        return Some(ParsingError::StackUnderflow);
                    }
                    if instruction_pointer <= 0. {
                        return Some(ParsingError::IllegalOperand);
                    } else {
                        // it's performing a relative swap; swaping <operand> and pop.
                        let a = self.stack[(stack_length - 1. - instruction_pointer) as usize];
                        let b = self.stack.pop().unwrap();
                        self.stack[(stack_length - 1. - instruction_pointer) as usize] = b;
                        self.stack.push(a);
                    }
                }
            }

            InstructionType::Add => {
                self.instruction_pointer += 1;

                if self.stack.len() < 2 {
                    return Some(ParsingError::StackUnderflow);
                }

                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a + b);
            }

            InstructionType::Subtract => {
                self.instruction_pointer += 1;

                if self.stack.len() < 2 {
                    return Some(ParsingError::StackUnderflow);
                }

                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a - b);
            }

            InstructionType::Multiply => {
                self.instruction_pointer += 1;

                if self.stack.len() < 2 {
                    return Some(ParsingError::StackUnderflow);
                }

                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a * b);
            }

            InstructionType::Divide => {
                self.instruction_pointer += 1;

                if self.stack.len() < 2 {
                    return Some(ParsingError::StackUnderflow);
                }

                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();

                if b == 0. {
                    return Some(ParsingError::DivisionByZero);
                }

                self.stack.push(a / b);
            }

            InstructionType::Equal => {
                self.instruction_pointer += 1;

                if self.stack.len() < 2 {
                    return Some(ParsingError::StackUnderflow);
                }

                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(((a == b) as Integer) as Float);
            }

            InstructionType::GreaterEqual => {
                self.instruction_pointer += 1;

                if self.stack.len() < 2 {
                    return Some(ParsingError::StackUnderflow);
                }

                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(((a >= b) as Integer) as Float);
            }

            InstructionType::Not => {
                self.instruction_pointer += 1;

                if self.stack.len() < 1 {
                    return Some(ParsingError::StackUnderflow);
                }

                let a = self.stack.pop().unwrap();
                let a = (!(a != 0.) as Integer) as Float;
                self.stack.push(a);
            }

            InstructionType::Jump => {
                if let Some(jump_to) = instruction.operand {
                    self.instruction_pointer = jump_to as usize;
                } else {
                    return Some(ParsingError::IllegalOperand);
                }
            }

            InstructionType::JumpIf => {
                self.instruction_pointer += 1;

                if self.stack.len() < 1 {
                    return Some(ParsingError::StackUnderflow);
                }

                let a = self.stack.pop().unwrap();
                self.stack.push(a);
                if let Some(jump_to) = instruction.operand {
                    if a != 0. {
                        self.instruction_pointer = jump_to as usize;
                    }
                } else {
                    return Some(ParsingError::IllegalOperand);
                }
            }

            InstructionType::Output => {
                self.instruction_pointer += 1;

                if self.stack.len() < 1 {
                    return Some(ParsingError::StackUnderflow);
                }

                let a = self.stack.pop().unwrap();
                println!("{}", a);
                self.stack.push(a);
            }

            InstructionType::Outputf => {
                self.instruction_pointer += 1;

                if self.stack.len() < 1 {
                    return Some(ParsingError::StackUnderflow);
                }

                let a = self.stack.pop().unwrap();
                println!("{:.15}", a);
                self.stack.push(a);
            }

            InstructionType::Dump => {
                self.instruction_pointer += 1;

                println!("stack: {:#?}", self.stack);
            }

            InstructionType::Halt => {
                self.halt = true;
            }
        }
        None
    }
}
