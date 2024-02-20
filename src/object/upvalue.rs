use crate::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Upvalue {
    location: Rc<RefCell<Value>>,
}

impl Upvalue {
    pub fn new(slot: &Rc<RefCell<Value>>) -> Self {
        Self {
            location: Rc::clone(slot),
        }
    }
}
