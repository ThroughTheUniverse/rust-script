use super::{parser::parse_rule::Precedence, Compiler};

mod parse_and_expression;
mod parse_binary_expression;
mod parse_dot_expression;
mod parse_fn_call_expression;
mod parse_grouping_expression;
mod parse_or_expression;
mod parse_self_expression;
mod parse_unary_expression;
mod parse_variable_expression;

impl Compiler {
    pub fn parse_expression(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }
}
