pub mod debug;
pub mod opcode;
use crate::value::{ConstantPool, Value};

pub struct Chunk {
    pub bytecodes: Vec<u8>,
    line_numbers: Vec<usize>,
    pub constant_pool: ConstantPool,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            bytecodes: Vec::new(),
            line_numbers: Vec::new(),
            constant_pool: ConstantPool::new(),
        }
    }

    pub fn push_bytecode<T: Into<u8>>(&mut self, byte: T, line_number: usize) {
        self.bytecodes.push(byte.into());
        self.line_numbers.push(line_number);
    }

    pub fn push_constant(&mut self, value: Value) -> u8 {
        self.constant_pool.push(value) as u8
    }
}
