pub mod opcode;
use crate::value::{Constants, Value};

pub struct Chunk {
    pub code: Vec<u8>,
    pub positions: Vec<usize>,
    pub constants: Constants,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            positions: Vec::new(),
            constants: Constants::new(),
        }
    }

    pub fn write_chunk(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: Value) {}
}
