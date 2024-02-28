use super::Compiler;
use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

impl Compiler {
    pub fn parse_let_declaration(&mut self) {
        use TokenKind::*;

        let global = self.parse_variable_name("Expect variable name.");
        if self.matches(Equal) {
            self.parse_expression();
        } else {
            self.emit_one_byte(OpCode::None);
        }
        self.consume(Semicolon, "Expect ';' after variable declaration.");
        self.define_variable(global);
    }

    pub fn parse_variable_name(&mut self, error_message: &str) -> u8 {
        self.consume(TokenKind::Identifier, error_message);

        self.declare_variable();
        if self.scope_depth > 0 {
            return 0;
        }
        let name = self.parser().previous.lexeme.clone();
        self.emit_identifier_constant(name)
    }

    pub fn define_variable(&mut self, global: u8) {
        if self.scope_depth > 0 {
            self.mark_initialized();
            return;
        }
        self.emit_two_bytes(OpCode::DefineGlobal, global);
    }

    pub fn declare_variable(&mut self) {
        if self.scope_depth == 0 {
            return;
        }
        let name = self.parser().previous.clone();
        for local in self.locals.iter().rev() {
            if local.depth.is_some() && local.depth.is_some_and(|depth| depth < self.scope_depth) {
                break;
            }

            if name == local.name {
                self.parser()
                    .error("Already a variable with this name in this scope.");
            }
        }
        self.add_local(name);
    }

    pub fn mark_initialized(&mut self) {
        if self.scope_depth == 0 {
            return;
        }
        let scope_depth = Some(self.scope_depth);
        self.locals.last_mut().unwrap().depth = scope_depth;
    }
}
