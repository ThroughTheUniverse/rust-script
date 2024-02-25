use crate::{
    chunk::{opcode::OpCode, Chunk},
    compiler::{Compiler, InterpretError},
    value::Value,
};

pub struct VirtualMachine {
    instruction_pointer: usize,
    stack: Vec<Value>,
    chunk: Chunk,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            instruction_pointer: 0,
            stack: Vec::new(),
            chunk: Chunk::new(),
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
                    return Ok(());
                }
                Constant => {
                    let constant = self.read_one_constant();
                    self.stack.push(constant);
                }
                None => self.stack.push(Value::None),
                True => self.stack.push(Value::Bool(true)),
                False => self.stack.push(Value::Bool(false)),
                Equal => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Bool(a == b));
                }
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
        if !self.peek(0).is_number() || !self.peek(1).is_number() {
            return self.runtime_error("Operands must be numbers");
        }
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
