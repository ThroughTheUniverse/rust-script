use crate::{
    chunk::opcode::OpCode, compiler::parser::parse_rule::Precedence, scanner::token::TokenKind,
};

use super::Compiler;

impl Compiler {
    pub fn parse_unary_expression(&mut self, _can_assign: bool) {
        use OpCode::*;
        use TokenKind::*;

        let unary_operator = self.parser().previous.kind;
        self.parse_precedence(Precedence::Unary);
        match unary_operator {
            Bang => self.emit_one_byte(Not),
            Minus => self.emit_one_byte(Negate),
            _ => todo!(),
        }
    }
}
