use std::{cell::RefCell, rc::Rc, time::SystemTime};

use crate::value::Value;

pub trait NativeFunction {
    fn call(&self, arg_count: usize, args: &[Value]) -> Value;
}

pub struct Clock {}

impl NativeFunction for Clock {
    fn call(&self, arg_count: usize, args: &[Value]) -> Value {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => Value::Number(n.as_millis() as f64),
            Err(_) => panic!("can't get system time"),
        }
    }
}
