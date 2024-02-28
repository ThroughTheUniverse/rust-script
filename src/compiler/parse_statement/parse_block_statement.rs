use super::Compiler;
use crate::scanner::token::TokenKind;

impl Compiler {
    pub fn parse_block_statement(&mut self) {
        use TokenKind::*;

        while !self.check(RightBrace) && !self.check(EOF) {
            self.parse_declaration();
        }

        self.consume(RightBrace, "Expect '}' after block.");
    }
}
