use crate::{chunk::opcode::OpCode, scanner::token::TokenKind};

use super::Compiler;

impl Compiler {
    pub fn parse_identifier(&mut self, can_assign: bool) {
        let name = self.parser().previous.lexeme.to_string();
        self.named_identifier(name, can_assign);
    }

    pub fn named_identifier(&mut self, name: String, can_assign: bool) {
        let get_opcode: OpCode;
        let set_opcode: OpCode;
        let mut arg = self.resolve_local(&name);
        if arg.is_some() {
            get_opcode = OpCode::GetLocal;
            set_opcode = OpCode::SetLocal;
        } else {
            let index = self.emit_identifier_constant(name);
            arg = Some(index);
            get_opcode = OpCode::GetGlobal;
            set_opcode = OpCode::SetGlobal;
        }

        if can_assign && self.matches(TokenKind::Equal) {
            self.parse_expression();
            self.emit_two_bytes(set_opcode, arg.unwrap());
        } else {
            self.emit_two_bytes(get_opcode, arg.unwrap());
        }
    }

    fn resolve_local(&mut self, name: &str) -> Option<u8> {
        for (i, local) in self.locals.iter().enumerate().rev() {
            if name == local.name.lexeme {
                if local.depth.is_none() {
                    self.parser()
                        .error("Can't read local variable in its own initializer.");
                }
                return Some(i as u8);
            }
        }

        None
    }
}
