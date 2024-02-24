use std::ops::Add;

use crate::{
    chunk::{opcode::OpCode, Chunk},
    compiler::{Compiler, InterpretError},
    value::Value,
};

pub struct VirtualMachine {
    instruction_pointer: usize,
    stack: Vec<Value>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            instruction_pointer: 0,
            stack: Vec::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), InterpretError> {
        let mut chunk = Chunk::new();
        let mut compiler = Compiler::new(&mut chunk);
        compiler.compile(source)?;
        self.instruction_pointer = 0;
        self.run(&chunk)
    }

    pub fn run(&mut self, chunk: &Chunk) -> Result<(), InterpretError> {
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

            let instruction: OpCode = self.read_one_bytecode(chunk).into();
            match instruction {
                Return => {
                    println!("{}", self.stack.pop().unwrap());
                    return Ok(());
                }
                Constant => {
                    let constant = self.read_one_constant(chunk);
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
                _ => return Err(InterpretError::RuntimeError),
            }
        }
    }

    fn read_one_bytecode(&mut self, chunk: &Chunk) -> u8 {
        let bytecode = chunk.bytecodes[self.instruction_pointer];
        self.instruction_pointer += 1;
        bytecode
    }

    fn read_one_constant(&mut self, chunk: &Chunk) -> Value {
        let index = self.read_one_bytecode(chunk);
        chunk.constant_pool.get(index as usize).clone()
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
