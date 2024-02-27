use super::function::Function;
use crate::value::Value;
use std::{fmt::Display, rc::Rc};

pub struct BoundMethod {
    pub receiver: Value,
    pub method: Rc<Function>,
}

impl BoundMethod {
    pub fn new(receiver: Value, method: Rc<Function>) -> Self {
        Self { receiver, method }
    }
}

impl Display for BoundMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.method)
    }
}
