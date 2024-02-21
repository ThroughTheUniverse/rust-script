use chunk::{opcode::OpCode, Chunk};
use scanner::Scanner;

mod chunk;
mod object;
mod scanner;
mod value;
mod vm;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Return.into());
}
