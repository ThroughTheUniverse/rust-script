pub mod debug;
pub mod opcode;
use crate::value::{ConstantPool, Value};

pub struct Chunk {
    code: Vec<u8>,
    line_numbers: Vec<usize>,
    constant_pool: ConstantPool,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            line_numbers: Vec::new(),
            constant_pool: ConstantPool::new(),
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: Value) {}
}
