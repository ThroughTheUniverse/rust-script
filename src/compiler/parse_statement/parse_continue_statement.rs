use crate::{compiler::Compiler, scanner::token::TokenKind};

impl Compiler {
    pub fn parse_continue_statement(&mut self) {
        if self.loop_start.is_none() {
            self.parser()
                .error("Cannot use 'continue' outside of a loop.");
        }

        self.consume(TokenKind::Semicolon, "Expect ';' after continue.");
        self.emit_loop(self.loop_start.unwrap());
    }
}
