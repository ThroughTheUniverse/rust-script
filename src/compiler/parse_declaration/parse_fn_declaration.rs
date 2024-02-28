use crate::{
    chunk::opcode::OpCode,
    compiler::{Compiler, FunctionKind},
    scanner::token::TokenKind,
    value::Value,
};
use std::rc::Rc;

impl Compiler {
    pub fn parse_fn_declaration(&mut self) {
        let global = self.parse_variable_name("Expect function name.");
        self.mark_initialized();
        self.parse_fn_body(FunctionKind::Function);
        self.define_variable(global);
    }

    pub fn parse_fn_body(&mut self, kind: FunctionKind) {
        use TokenKind::*;

        let mut compiler = self.fork(kind);
        compiler.begin_scope();
        compiler.consume(LeftParen, "Expect '(' after function name.");
        if !compiler.check(RightParen) {
            loop {
                compiler.function.arity += 1;
                if compiler.function.arity > u8::MAX.into() {
                    compiler
                        .parser()
                        .error_at_current("Can't have more than 255 parameters.");
                }
                let constant = compiler.parse_variable_name("Expect parameter name.");
                compiler.define_variable(constant);

                if !compiler.matches(Comma) {
                    break;
                }
            }
        }
        compiler.consume(RightParen, "Expect ')' after parameters.");
        compiler.consume(LeftBrace, "Expect '{' before function body.");
        compiler.parse_block_statement();
        let function = compiler.end_complier();
        let value = self.make_constant(Value::Function(Rc::new(function)));
        self.emit_two_bytes(OpCode::Constant, value)
    }
}
