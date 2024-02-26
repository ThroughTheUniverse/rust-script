use std::{collections::HashMap, rc::Rc, result};

use crate::{
    chunk::{opcode::OpCode, Chunk},
    compiler::{Compiler, FunctionKind, InterpretError},
    object::{
        function::{self, Function},
        native_function::{Clock, NativeFunction},
    },
    value::Value,
};

struct CallFrame {
    function: Rc<Function>,
    ip: usize,
    base_slot: usize,
}

pub struct VirtualMachine {
    frames: Vec<CallFrame>,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let mut result = Self {
            frames: Vec::new(),
            stack: Vec::new(),
            globals: HashMap::new(),
        };
        let clock = Rc::new(Clock {});
        result.define_native("clock", clock.clone());
        result
    }

    fn current_frame(&mut self) -> &mut CallFrame {
        self.frames.last_mut().unwrap()
    }

    pub fn interpret(&mut self, source: &str) -> Result<(), InterpretError> {
        let compiler = Compiler::new(FunctionKind::Script);

        let function = Rc::new(compiler.compile(source)?);
        self.stack.push(Value::Function(function.clone()));
        self.call(function.clone(), 0);
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
                Jump => {
                    let offset = self.read_two_bytecodes();
                    self.current_frame().ip += offset as usize;
                }
                JumpIfFalse => {
                    let offset = self.read_two_bytecodes();
                    if self.peek(0).is_falsey() {
                        self.current_frame().ip += offset as usize;
                    }
                }
                Loop => {
                    let offset = self.read_two_bytecodes();
                    self.current_frame().ip -= offset as usize;
                }
                Call => {
                    let arg_count = self.read_one_bytecode();
                    if !self.call_value(self.peek(arg_count as usize), arg_count) {
                        return Err(InterpretError::RuntimeError);
                    }
                }
                Return => {
                    let result = self.stack.pop().unwrap();
                    let frame = self.frames.pop().unwrap();
                    if self.frames.len() == 0 {
                        self.stack.pop();
                        return Ok(());
                    }
                    self.stack.truncate(frame.base_slot);
                    self.stack.push(result);
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
                    let index = self.current_frame().base_slot + slot as usize;
                    let value = self.stack[index].clone();
                    self.stack.push(value);
                }
                SetLocal => {
                    let slot = self.read_one_bytecode();
                    let index = self.current_frame().base_slot + slot as usize;
                    self.stack[index] = self.peek(0);
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

    fn current_chunk(&mut self) -> &Chunk {
        &self.current_frame().function.chunk
    }

    fn read_one_bytecode(&mut self) -> u8 {
        let ip = self.current_frame().ip;
        let bytecode = self.current_frame().function.chunk.bytecodes[ip];
        self.current_frame().ip += 1;
        bytecode
    }

    fn read_two_bytecodes(&mut self) -> u16 {
        self.current_frame().ip += 2;
        let high_index = self.current_frame().ip - 2;
        let low_index = self.current_frame().ip - 1;
        let high_byte = self.current_chunk().bytecodes[high_index];
        let low_byte = self.current_chunk().bytecodes[low_index];
        ((high_byte as u16) << 8) | low_byte as u16
    }

    fn read_one_constant(&mut self) -> Value {
        let index = self.read_one_bytecode();
        self.current_chunk()
            .constant_pool
            .get(index as usize)
            .clone()
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

    fn call_value(&mut self, callee: Value, arg_count: u8) -> bool {
        use Value::*;
        match callee {
            Function(function) => return self.call(function.clone(), arg_count),
            NativeFunction(function) => {
                let stack_top = self.stack.len();
                let result = function.call(
                    arg_count as usize,
                    &self.stack[stack_top - arg_count as usize..stack_top],
                );
                self.stack.truncate(stack_top - (arg_count + 1) as usize);
                self.stack.push(result);
                return true;
            }
            _ => (),
        }
        let _ = self.runtime_error("Can only call functions and classes.");
        false
    }

    fn define_native(&mut self, name: &str, function: Rc<dyn NativeFunction>) {
        self.globals
            .insert(name.to_string(), Value::NativeFunction(function.clone()));
    }

    fn call(&mut self, function: Rc<Function>, arg_count: u8) -> bool {
        if arg_count as usize != function.arity {
            let _ = self.runtime_error(&format!(
                "Expected {} arguments but got {}.",
                function.arity, arg_count
            ));
            return false;
        }

        if self.frames.len() == u8::MAX.into() {
            let _ = self.runtime_error("Stack overflow.");
            return false;
        }

        let base_slot = self.stack.len() - arg_count as usize - 1;
        self.frames.push(CallFrame {
            function: function.clone(),
            ip: 0,
            base_slot,
        });
        true
    }

    fn runtime_error(&mut self, message: &str) -> Result<(), InterpretError> {
        eprintln!("{}", message);
        for frame in self.frames.iter().rev() {
            let index = frame.ip - 1;
            let line_number = frame.function.chunk.line_numbers[index];
            eprint!("[line {}] in ", line_number);
            if frame.function.name.is_empty() {
                eprintln!("script");
            } else {
                eprintln!("{}()", frame.function.name);
            }
        }
        self.stack = Vec::new();
        self.frames = Vec::new();
        Err(InterpretError::RuntimeError)
    }
}
