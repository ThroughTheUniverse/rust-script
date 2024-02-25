use crate::scanner::token::TokenKind;

use super::Compiler;

impl<'a> Compiler<'a> {
    pub fn parse_block(&mut self) {
        while !self.check(TokenKind::RightBrace) && !self.check(TokenKind::EOF) {
            self.parse_declaration();
        }

        self.consume(TokenKind::RightBrace, "Expect '}' after block.");
    }
}
