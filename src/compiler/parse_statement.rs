use crate::scanner::token::TokenKind;

use super::Compiler;

impl<'a> Compiler<'a> {
    pub fn parse_statement(&mut self) {
        if self.matches(TokenKind::Print) {
            self.parse_print();
        } else if self.matches(TokenKind::LeftBrace) {
            self.begin_scope();
            self.parse_block();
            self.end_scope();
        } else {
            self.parse_expression_statement();
        }
    }
}
