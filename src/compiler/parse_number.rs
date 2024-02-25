use crate::value::Value;

use super::Compiler;

impl<'a> Compiler<'a> {
    pub fn parse_number(&mut self, _can_assign: bool) {
        let number = self.parser.previous.lexeme.parse::<f64>().unwrap();
        self.emit_constant(Value::Number(number));
    }
}
