use super::closure::Closure;
use crate::value::Value;
use std::rc::Rc;

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
