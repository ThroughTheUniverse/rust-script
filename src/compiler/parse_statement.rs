use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl Compiler {
    pub fn parse_statement(&mut self) {
        if self.matches(TokenKind::Print) {
            self.parse_print();
        } else if self.matches(TokenKind::For) {
            self.parse_for();
        } else if self.matches(TokenKind::If) {
            self.parse_if();
        } else if self.matches(TokenKind::While) {
            self.parse_while();
        } else if self.matches(TokenKind::LeftBrace) {
            self.begin_scope();
            self.parse_block();
            self.end_scope();
        } else {
            self.parse_expression_statement();
        }
    }
}
