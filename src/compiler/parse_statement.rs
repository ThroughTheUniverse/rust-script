use crate::scanner::token::TokenKind;

use super::Compiler;

impl<'a> Compiler<'a> {
    pub fn parse_statement(&mut self) {
        if self.matches(TokenKind::Print) {
            self.parse_print();
        } else {
            self.parse_expression_statement();
        }
    }
}
