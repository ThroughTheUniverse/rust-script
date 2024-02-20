use std::{cell::RefCell, rc::Rc};

use crate::value::Value;

pub trait NativeFunction {
    fn call(&self, arg_count: usize, args: &[Rc<RefCell<Value>>]) -> Value;
}
