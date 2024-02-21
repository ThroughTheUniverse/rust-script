pub mod debug;
pub mod opcode;
use crate::value::{Constants, Value};

pub struct Chunk {
    code: Vec<u8>,
    positions: Vec<usize>,
    constants: Constants,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            positions: Vec::new(),
            constants: Constants::new(),
        }
    }

    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: Value) {}
}
