use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl<'a> Compiler<'a> {
    pub fn parse_binary(&mut self, _can_assign: bool) {
        use OpCode::*;
        use TokenKind::*;
        let binary_operator = self.parser().previous.kind;
        let precedence: u8 = self.rules().get(binary_operator).precedence.into();
        self.parse_precedence((precedence + 1).into());

        match binary_operator {
            BangEqual => self.emit_two_bytes(OpCode::Equal, Not),
            EqualEqual => self.emit_one_byte(OpCode::Equal),
            TokenKind::Greater => self.emit_one_byte(OpCode::Greater),
            GreaterEqual => self.emit_two_bytes(OpCode::Less, Not),
            TokenKind::Less => self.emit_one_byte(OpCode::Less),
            LessEqual => self.emit_two_bytes(OpCode::Greater, Not),
            Plus => self.emit_one_byte(Add),
            Minus => self.emit_one_byte(Subtract),
            Star => self.emit_one_byte(Multiply),
            Slash => self.emit_one_byte(Divide),
            _ => todo!(),
        }
    }
}
