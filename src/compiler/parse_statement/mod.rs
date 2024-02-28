use crate::scanner::token::TokenKind;

use super::Compiler;

mod parse_block_statement;
mod parse_expression_statement;
mod parse_for_statement;
mod parse_if_statement;
mod parse_print_statement;
mod parse_return_statement;
mod parse_while_statement;

impl Compiler {
    pub fn parse_statement(&mut self) {
        if self.matches(TokenKind::Print) {
            self.parse_print_statement();
        } else if self.matches(TokenKind::For) {
            self.parse_for_statement();
        } else if self.matches(TokenKind::If) {
            self.parse_if_statement();
        } else if self.matches(TokenKind::Return) {
            self.parse_return_statement();
        } else if self.matches(TokenKind::While) {
            self.parse_while_statement();
        } else if self.matches(TokenKind::LeftBrace) {
            self.begin_scope();
            self.parse_block_statement();
            self.end_scope();
        } else {
            self.parse_expression_statement();
        }
    }
}
