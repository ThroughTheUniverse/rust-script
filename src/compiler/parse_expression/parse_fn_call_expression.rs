use super::Compiler;
use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

impl Compiler {
    pub fn parse_fn_call_expression(&mut self, _can_assign: bool) {
        let arg_count = self.argument_list();
        self.emit_two_bytes(OpCode::Call, arg_count);
    }

    pub fn argument_list(&mut self) -> u8 {
        use TokenKind::*;

        let mut arg_count = 0;
        if !self.check(RightParen) {
            loop {
                self.parse_expression();
                if arg_count == u8::MAX {
                    self.parser().error("Can't have more than 255 arguments.");
                }
                arg_count += 1;
                if !self.matches(Comma) {
                    break;
                }
            }
        }

        self.consume(RightParen, "Expect ')' after arguments.");
        arg_count
    }
}
