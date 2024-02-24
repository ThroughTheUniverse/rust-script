use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl<'a> Compiler<'a> {
    pub fn parse_binary(&mut self) {
        use OpCode::*;
        use TokenKind::*;
        let binary_operator = self.parser.previous.kind;
        let precedence: u8 = self.rules.get(&binary_operator).unwrap().precedence.into();
        self.parse_precedence((precedence + 1).into());

        match binary_operator {
            Plus => self.emit_one_byte(Add),
            Minus => self.emit_one_byte(Subtract),
            Star => self.emit_one_byte(Multiply),
            Slash => self.emit_one_byte(Divide),
            _ => todo!(),
        }
    }
}
