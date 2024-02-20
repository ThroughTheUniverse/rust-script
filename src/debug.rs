use crate::chunk::opcode::OpCode;
use crate::{chunk::Chunk, value::Constants};
use std::io::{self, Write};

// pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
//     println!("== {} ==", name);
//     let mut offset = 0;
//     while offset < chunk.code.len() {
//         offset = disassemble_instruction(chunk, offset);
//     }
//     ()
// }

// // fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
// //     print!("{:04} ", offset);
// //     io::stdout().flush().unwrap();

// //     let instruction: OpCode = chunk.code.get(offset).unwrap().to_owned().into();

// //     match instruction {
// //         OpCoReturn => simple_instruction(&Return.to_string(), offset),
// //         _ => {
// //             println!("Unknown opcode {}", instruction);
// //             offset + 1
// //         }
// //     }
// // }

// fn simple_instruction(name: &str, offset: usize) -> usize {
//     println!("{name}");
//     offset + 1
// }

// fn constant_instruction(name: &str, chunk: &Chunk, offset: usize) -> usize {
//     let constant = chunk.code.get(offset + 1).unwrap();
//     print!("{:<16} {:>4} '", name, constant);
//     1
// }
