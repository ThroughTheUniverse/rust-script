use std::{
    io::{self, Write},
    ops::{BitOr, Shl},
};

use crate::chunk::{opcode::OpCode, Chunk};

enum JumpDirection {
    Forward,
    Backward,
}

impl Chunk {
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code.get(offset + 1).unwrap().to_owned();
        print!("{:<16} {:>4} '", name, constant);
        self.constants.print_nth(constant as usize);
        println!("'");
        offset + 2
    }

    fn invoke_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code.get(offset + 1).unwrap().to_owned();
        let arg_count = self.code.get(offset + 2).unwrap().to_owned();
        print!("{:<16} ({} args) {:>4} '", name, arg_count, constant);
        self.constants.print_nth(constant as usize);
        println!("'");
        offset + 3
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn byte_instruction(&self, name: &str, offset: usize) -> usize {
        let slot = self.code.get(offset + 1).unwrap().to_owned();
        println!("{:<16} {:>4}", name, slot);
        offset + 2
    }

    fn jump_instruction(&self, name: &str, direction: JumpDirection, offset: usize) -> usize {
        use JumpDirection::*;
        let jump = (self.code.get(offset + 1).unwrap().to_owned() as usize).shl(8)
            | (self.code.get(offset + 2).unwrap().to_owned() as usize);
        let jump_to = match direction {
            Forward => offset + 3 + jump,
            Backward => offset + 3 - jump,
        };
        println!("{:<16} {:>4} -> {}", name, offset, jump_to);
        offset + 3
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        use crate::chunk::opcode::OpCode::*;
        use crate::value::Value::Function;
        use JumpDirection::*;

        print!("{:04} ", offset);

        if offset > 0
            && self.positions.get(offset).unwrap().to_owned()
                == self.positions.get(offset - 1).unwrap().to_owned()
        {
            print!("   | ");
        } else {
            print!("{:4} ", self.positions.get(offset).unwrap().to_owned());
        }

        let instruction: OpCode = self.code.get(offset).unwrap().to_owned().into();
        match instruction {
            Constant => self.constant_instruction(Constant.to_string().as_str(), offset),
            Return => self.simple_instruction(Return.to_string().as_str(), offset),
            Negate => self.simple_instruction(Negate.to_string().as_str(), offset),
            Add => self.simple_instruction(Add.to_string().as_str(), offset),
            Subtract => self.simple_instruction(Subtract.to_string().as_str(), offset),
            Multiply => self.simple_instruction(Multiply.to_string().as_str(), offset),
            Divide => self.simple_instruction(Divide.to_string().as_str(), offset),
            None => self.simple_instruction(None.to_string().as_str(), offset),
            True => self.simple_instruction(True.to_string().as_str(), offset),
            False => self.simple_instruction(False.to_string().as_str(), offset),
            Not => self.simple_instruction(Not.to_string().as_str(), offset),
            Equal => self.simple_instruction(Equal.to_string().as_str(), offset),
            Greater => self.simple_instruction(Greater.to_string().as_str(), offset),
            Less => self.simple_instruction(Less.to_string().as_str(), offset),
            Print => self.simple_instruction(Print.to_string().as_str(), offset),
            Pop => self.simple_instruction(Pop.to_string().as_str(), offset),
            DefineGlobal => self.constant_instruction(DefineGlobal.to_string().as_str(), offset),
            GetGlobal => self.constant_instruction(GetGlobal.to_string().as_str(), offset),
            SetGlobal => self.constant_instruction(SetGlobal.to_string().as_str(), offset),
            GetLocal => self.byte_instruction(GetLocal.to_string().as_str(), offset),
            SetLocal => self.byte_instruction(SetLocal.to_string().as_str(), offset),
            JumpIfFalse => self.jump_instruction(JumpIfFalse.to_string().as_str(), Forward, offset),
            Jump => self.jump_instruction(Jump.to_string().as_str(), Forward, offset),
            Loop => self.jump_instruction(Loop.to_string().as_str(), Backward, offset),
            Call => self.byte_instruction(Call.to_string().as_str(), offset),
            Closure => {
                let mut i = offset + 1;
                let constant = self.code.get(offset).unwrap().to_owned();
                i += 1;
                print!("{:-16} {:4} ", Closure.to_string(), constant);
                self.constants.print_nth(constant as usize);
                println!();
                if let Function(function) = self.constants.values.get(constant as usize).unwrap() {
                    for _ in 0..function.upvalue_count {
                        let is_local: &str = if self.code[i] == 0 {
                            "upvalue"
                        } else {
                            "local"
                        };
                        i += 1;
                        let index = self.code[i];
                        i += 1;
                        println!("{:04}      |                     {is_local} {index}", i - 2);
                    }
                } else {
                    panic!("No function at position {constant}");
                }
                i
            }
            GetUpvalue => self.byte_instruction(GetUpvalue.to_string().as_str(), offset),
            SetUpvalue => self.byte_instruction(SetUpvalue.to_string().as_str(), offset),
            CloseUpvalue => self.simple_instruction(CloseUpvalue.to_string().as_str(), offset),
            Struct => self.constant_instruction(Struct.to_string().as_str(), offset),
            GetProperty => self.constant_instruction(GetProperty.to_string().as_str(), offset),
            SetProperty => self.constant_instruction(SetProperty.to_string().as_str(), offset),
            Method => self.constant_instruction(Method.to_string().as_str(), offset),
            Invoke => self.invoke_instruction(Invoke.to_string().as_str(), offset),
            Inherit => self.simple_instruction(Inherit.to_string().as_str(), offset),
            GetSuper => self.constant_instruction(GetSuper.to_string().as_str(), offset),
            SuperInvoke => self.invoke_instruction(SuperInvoke.to_string().as_str(), offset),
            _ => panic!("Unknown Opcode"),
        }
    }
}
