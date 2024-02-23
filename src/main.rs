use chunk::{opcode::OpCode, Chunk};
use scanner::Scanner;
use vm::VirtualMachine;

mod chunk;
mod object;
mod scanner;
mod value;
mod vm;

fn main() {
    let mut chunk = Chunk::new();
    let constant = chunk.push_constant(value::Value::Number(3.0));
    chunk.push_bytecode(OpCode::Constant, 123);
    chunk.push_bytecode(constant, 123);

    let constant = chunk.push_constant(value::Value::Number(3.0));
    chunk.push_bytecode(OpCode::Constant, 123);
    chunk.push_bytecode(constant, 123);

    chunk.push_bytecode(OpCode::Add, 123);

    let constant = chunk.push_constant(value::Value::Number(2.0));
    chunk.push_bytecode(OpCode::Constant, 123);
    chunk.push_bytecode(constant, 123);

    chunk.push_bytecode(OpCode::Divide, 123);
    chunk.push_bytecode(OpCode::Negate, 123);

    chunk.push_bytecode(OpCode::Return, 123);
    chunk.disassemble_chunk("test chunk");

    let mut vm = VirtualMachine::new(&chunk);
    vm.run();
}
