use crate::scanner::token::TokenKind;

use super::Compiler;

impl Compiler {
    pub fn parse_declaration(&mut self) {
        if self.matches(TokenKind::Fn) {
            self.parse_fn();
        } else if self.matches(TokenKind::Let) {
            self.parse_let();
        } else {
            self.parse_statement();
        }
        if self.parser().is_panic_mode.get() {
            self.synchronize();
        }
    }
}
