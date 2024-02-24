use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::{parse_rule::Precedence, Compiler};

impl<'a> Compiler<'a> {
    pub fn parse_unary(&mut self) {
        use OpCode::*;
        use TokenKind::*;

        let unary_operator = self.parser.previous.kind;
        self.parse_precedence(Precedence::Unary);
        match unary_operator {
            Minus => self.emit_one_byte(Negate),
            _ => todo!(),
        }
    }
}
