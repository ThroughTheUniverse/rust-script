use std::ops::Add;

use crate::{
    chunk::{opcode::OpCode, Chunk},
    value::Value,
};

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VirtualMachine<'a> {
    chunk: &'a Chunk,
    instruction_pointer: usize,
    stack: Vec<Value>,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        Self {
            chunk,
            instruction_pointer: 0,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> InterpretResult {
        use InterpretResult::*;
        use OpCode::*;
        loop {
            #[cfg(feature = "debug_trace_execution")]
            {
                print!("          ");
                self.stack.iter().for_each(|value| {
                    print!("[ ");
                    println!("{value}");
                    print!(" ]");
                });
                println!();
                chunk.disassemble_instruction(self.ip);
            }

            let instruction: OpCode = self.read_one_bytecode().into();
            match instruction {
                Return => {
                    println!("{}", self.stack.pop().unwrap());
                    return Ok;
                }
                Constant => {
                    let constant = self.read_one_constant();
                    self.stack.push(constant);
                }
                Negate => {
                    let value = self.stack.pop().unwrap();
                    self.stack.push(-value);
                }
                Add => self.binary_operator(Add),
                Subtract => self.binary_operator(Subtract),
                Multiply => self.binary_operator(Multiply),
                Divide => self.binary_operator(Divide),
                _ => return RuntimeError,
            }
        }
    }

    fn read_one_bytecode(&mut self) -> u8 {
        let bytecode = self.chunk.bytecodes[self.instruction_pointer];
        self.instruction_pointer += 1;
        bytecode
    }

    fn read_one_constant(&mut self) -> Value {
        let index = self.read_one_bytecode();
        self.chunk.constant_pool.get(index as usize).clone()
    }

    fn binary_operator(&mut self, operator: OpCode) {
        use OpCode::*;
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        let result = match operator {
            Add => a + b,
            Subtract => a - b,
            Multiply => a * b,
            Divide => a / b,
            _ => panic!(""),
        };
        self.stack.push(result);
    }
}
