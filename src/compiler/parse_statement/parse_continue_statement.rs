use crate::{compiler::Compiler, scanner::token::TokenKind};

impl Compiler {
    pub fn parse_continue_statement(&mut self) {
        self.consume(TokenKind::Semicolon, "Expect ';' after continue.");
        self.emit_loop(self.loop_start);
    }
}
