use super::Compiler;
use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

mod parse_number_literal;
mod parse_string_literal;

impl Compiler {
    pub fn parser_literal(&mut self, _can_assign: bool) {
        use TokenKind::*;

        let kind = self.parser().previous.kind;
        match kind {
            False => self.emit_one_byte(OpCode::False),
            None => self.emit_one_byte(OpCode::None),
            True => self.emit_one_byte(OpCode::True),
            _ => panic!("literal not found"),
        }
    }
}
