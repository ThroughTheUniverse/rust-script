use crate::chunk::{opcode::OpCode, Chunk};
use std::ops::Shl;

enum JumpDirection {
    Forward,
    Backward,
}

impl Chunk {
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {name} ==");
        let mut offset = 0;
        while offset < self.bytecodes.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        use crate::chunk::opcode::OpCode::*;
        use JumpDirection::*;

        print!("{offset:04} ");

        if offset > 0 && self.line_numbers[offset] == self.line_numbers[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.line_numbers[offset]);
        }

        let instruction: OpCode = self.bytecodes[offset].into();
        match instruction {
            // constant
            Constant => self.constant_instruction(Constant.to_string().as_str(), offset),
            // literals
            None => self.simple_instruction(None.to_string().as_str(), offset),
            True => self.simple_instruction(True.to_string().as_str(), offset),
            False => self.simple_instruction(False.to_string().as_str(), offset),
            // global variable pop
            Pop => self.simple_instruction(Pop.to_string().as_str(), offset),
            End => self.simple_instruction(End.to_string().as_str(), offset),
            // local variable
            GetLocal => self.byte_instruction(GetLocal.to_string().as_str(), offset),
            SetLocal => self.byte_instruction(SetLocal.to_string().as_str(), offset),
            // global variable
            GetGlobal => self.constant_instruction(GetGlobal.to_string().as_str(), offset),
            SetGlobal => self.constant_instruction(SetGlobal.to_string().as_str(), offset),
            DefineGlobal => self.constant_instruction(DefineGlobal.to_string().as_str(), offset),
            // property
            GetProperty => self.constant_instruction(GetProperty.to_string().as_str(), offset),
            SetProperty => self.constant_instruction(SetProperty.to_string().as_str(), offset),
            // comparison
            Equal => self.simple_instruction(Equal.to_string().as_str(), offset),
            Greater => self.simple_instruction(Greater.to_string().as_str(), offset),
            Less => self.simple_instruction(Less.to_string().as_str(), offset),
            // binary
            Add => self.simple_instruction(Add.to_string().as_str(), offset),
            Subtract => self.simple_instruction(Subtract.to_string().as_str(), offset),
            Multiply => self.simple_instruction(Multiply.to_string().as_str(), offset),
            Divide => self.simple_instruction(Divide.to_string().as_str(), offset),
            Modulo => self.simple_instruction(Modulo.to_string().as_str(), offset),
            Power => self.simple_instruction(Power.to_string().as_str(), offset),
            // not
            Not => self.simple_instruction(Not.to_string().as_str(), offset),
            // negate
            Negate => self.simple_instruction(Negate.to_string().as_str(), offset),
            // print
            Print => self.simple_instruction(Print.to_string().as_str(), offset),
            // jump
            Jump => self.jump_instruction(Jump.to_string().as_str(), Forward, offset),
            JumpIfFalse => self.jump_instruction(JumpIfFalse.to_string().as_str(), Forward, offset),
            // loop
            Loop => self.jump_instruction(Loop.to_string().as_str(), Backward, offset),
            // invoke
            Invoke => self.invoke_instruction(Invoke.to_string().as_str(), offset),
            Call => self.byte_instruction(Call.to_string().as_str(), offset),
            Return => self.simple_instruction(Return.to_string().as_str(), offset),
            // struct
            Struct => self.constant_instruction(Struct.to_string().as_str(), offset),
            Method => self.constant_instruction(Method.to_string().as_str(), offset),
            _ => panic!("Unknown Opcode"),
        }
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let index = self.bytecodes[offset + 1];
        print!("{name:<16} {index:>4} '");
        self.constant_pool.print_nth(index as usize);
        println!("'");
        offset + 2
    }

    fn invoke_instruction(&self, name: &str, offset: usize) -> usize {
        let index = self.bytecodes[offset + 1];
        let arg_count = self.bytecodes[offset + 2];
        print!("{name:<16} ({arg_count} args) {index:>4} '");
        self.constant_pool.print_nth(index as usize);
        println!("'");
        offset + 3
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{name}");
        offset + 1
    }

    fn byte_instruction(&self, name: &str, offset: usize) -> usize {
        let slot = self.bytecodes[offset + 1];
        println!("{name:<16} {slot:>4}");
        offset + 2
    }

    fn jump_instruction(&self, name: &str, direction: JumpDirection, offset: usize) -> usize {
        use JumpDirection::*;
        let jump =
            (self.bytecodes[offset + 1] as usize).shl(8) | (self.bytecodes[offset + 2] as usize);
        let jump = match direction {
            Forward => offset + 3 + jump,
            Backward => offset + 3 - jump,
        };
        println!("{name:<16} {offset:>4} -> {jump}");
        offset + 3
    }
}
