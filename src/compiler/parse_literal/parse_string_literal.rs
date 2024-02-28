use crate::value::Value;

use super::Compiler;

impl Compiler {
    pub fn parser_string_literal(&mut self, _can_assign: bool) {
        let end = self.parser().previous.lexeme.len() - 1;
        let string = self.parser().previous.lexeme[1..end].to_string();
        self.emit_constant(Value::String(string));
    }
}
