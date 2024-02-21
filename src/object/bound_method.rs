use super::closure::Closure;
use crate::value::Value;
use std::{fmt::Display, rc::Rc};

pub struct BoundMethod {
    receiver: Value,
    method: Rc<Closure>,
}

impl BoundMethod {
    pub fn new(receiver: &Value, method: &Rc<Closure>) -> Self {
        Self {
            receiver: receiver.clone(),
            method: Rc::clone(method),
        }
    }
}

impl Display for BoundMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.method)
    }
}
