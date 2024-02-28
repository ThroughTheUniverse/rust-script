use super::function_object::FunctionObject;
use crate::value::Value;
use std::{fmt::Display, rc::Rc};

pub struct BoundMethodObject {
    pub receiver: Value,
    pub method: Rc<FunctionObject>,
}

impl BoundMethodObject {
    pub fn new(receiver: Value, method: Rc<FunctionObject>) -> Self {
        Self { receiver, method }
    }
}

impl Display for BoundMethodObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.method)
    }
}
