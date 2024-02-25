use std::collections::HashMap;

use crate::{
    chunk::{opcode::OpCode, Chunk},
    compiler::{Compiler, InterpretError},
    value::Value,
};

pub struct VirtualMachine {
    instruction_pointer: usize,
    stack: Vec<Value>,
    chunk: Chunk,
    globals: HashMap<String, Value>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            instruction_pointer: 0,
            stack: Vec::new(),
            chunk: Chunk::new(),
            globals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), InterpretError> {
        let mut chunk = Chunk::new();
        let mut compiler = Compiler::new(&mut chunk);

        compiler.compile(source)?;

        self.instruction_pointer = 0;
        self.chunk = chunk;
        self.run()
    }

    pub fn run(&mut self) -> Result<(), InterpretError> {
        use OpCode::*;
        loop {
            // {
            //     print!("          ");
            //     self.stack.iter().for_each(|value| {
            //         print!("[ ");
            //         println!("{value}");
            //         print!(" ]");
            //     });
            //     println!();
            //     self.chunk.disassemble_instruction(self.instruction_pointer);
            // }

            let instruction: OpCode = self.read_one_bytecode().into();
            match instruction {
                Print => {
                    println!("{}", self.stack.pop().unwrap());
                }
                Return => {
                    return Ok(());
                }
                Constant => {
                    let constant = self.read_one_constant();
                    self.stack.push(constant);
                }
                None => self.stack.push(Value::None),
                True => self.stack.push(Value::Bool(true)),
                False => self.stack.push(Value::Bool(false)),
                Pop => {
                    self.stack.pop();
                }
                GetLocal => {
                    let slot = self.read_one_bytecode();
                    let value = self.stack[slot as usize].clone();
                    self.stack.push(value);
                }
                SetLocal => {
                    let slot = self.read_one_bytecode();
                    self.stack[slot as usize] = self.peek(0);
                }
                GetGlobal => {
                    if let Value::String(name) = self.read_one_constant() {
                        if let Some(value) = self.globals.get(&name) {
                            self.stack.push(value.clone());
                        } else {
                            return self.runtime_error(&format!("Undefined variable '{}'", name));
                        }
                    } else {
                        return self.runtime_error("No identifier name");
                    }
                }
                DefineGlobal => {
                    if let Value::String(name) = self.read_one_constant() {
                        self.globals.insert(name, self.peek(0));
                        self.stack.pop();
                    } else {
                        return self.runtime_error("No identifier name");
                    }
                }
                SetGlobal => {
                    if let Value::String(name) = self.read_one_constant() {
                        if let Option::None = self.globals.insert(name.clone(), self.peek(0)) {
                            self.globals.remove(&name);
                            return self.runtime_error(&format!("Undefined variable '{}'", name));
                        }
                    } else {
                        return self.runtime_error("No identifier name");
                    }
                }
                Equal => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(a == b));
                }
                Greater => self.binary_operator(Greater)?,
                Less => self.binary_operator(Less)?,
                Add => self.binary_operator(Add)?,
                Subtract => self.binary_operator(Subtract)?,
                Multiply => self.binary_operator(Multiply)?,
                Divide => self.binary_operator(Divide)?,
                Not => {
                    let value = self.stack.pop().unwrap().is_falsey();
                    self.stack.push(Value::Bool(value));
                }
                Negate => {
                    if !self.peek(0).is_number() {
                        return self.runtime_error("Operand must be a number.");
                    }

                    let value = self.stack.pop().unwrap();
                    self.stack.push(-value);
                }
                _ => return Err(InterpretError::RuntimeError),
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

    fn binary_operator(&mut self, operator: OpCode) -> Result<(), InterpretError> {
        use OpCode::*;
        if (self.peek(0).is_string() && self.peek(1).is_string())
            || (self.peek(0).is_number() && self.peek(1).is_number())
        {
            let b = self.stack.pop().unwrap();
            let a = self.stack.pop().unwrap();
            let result = match operator {
                Add => a + b,
                Subtract => a - b,
                Multiply => a * b,
                Divide => a / b,
                Greater => Value::Bool(a > b),
                Less => Value::Bool(a < b),
                _ => return Err(InterpretError::RuntimeError),
            };
            self.stack.push(result);
            Ok(())
        } else {
            self.runtime_error("Operands must be two numbers or two strings.")
        }
    }

    fn peek(&self, distance: usize) -> Value {
        let stack_top_index = self.stack.len() - 1;
        self.stack[stack_top_index - distance].clone()
    }

    fn runtime_error(&mut self, message: &str) -> Result<(), InterpretError> {
        let line_number = self.chunk.line_numbers[self.instruction_pointer - 1];
        eprintln!("{}", message);
        eprintln!("[line {line_number}] in script");
        self.stack = Vec::new();
        Err(InterpretError::RuntimeError)
    }
}
