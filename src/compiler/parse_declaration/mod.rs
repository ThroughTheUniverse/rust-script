use crate::scanner::token::TokenKind;

use super::Compiler;

mod parse_fn_declaration;
mod parse_let_declaration;
mod parse_struct_declaration;

impl Compiler {
    pub fn parse_declaration(&mut self) {
        if self.matches(TokenKind::Fn) {
            self.parse_fn_declaration();
        } else if self.matches(TokenKind::Struct) {
            self.parse_struct_declaration();
        } else if self.matches(TokenKind::Let) {
            self.parse_let_declaration();
        } else {
            self.parse_statement();
        }
        if self.parser().is_panic_mode.get() {
            self.synchronize();
        }
    }
}
