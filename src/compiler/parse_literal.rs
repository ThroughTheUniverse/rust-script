use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl<'a> Compiler<'a> {
    pub fn parser_literal(&mut self) {
        use TokenKind::*;
        match self.parser.previous.kind {
            False => self.emit_one_byte(OpCode::False),
            None => self.emit_one_byte(OpCode::None),
            True => self.emit_one_byte(OpCode::True),
            _ => panic!("can not parse literal"),
        }
    }
}
