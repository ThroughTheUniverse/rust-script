use std::time::SystemTime;

use crate::value::Value;

pub trait NativeFunctionObject {
    fn call(&self, arg_count: usize, args: &[Value]) -> Value;
}

pub struct Clock {}

impl NativeFunctionObject for Clock {
    fn call(&self, _arg_count: usize, _args: &[Value]) -> Value {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => Value::Number(n.as_millis() as f64),
            Err(_) => panic!("can't get system time"),
        }
    }
}
