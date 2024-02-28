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

pub struct Println {}

impl NativeFunctionObject for Println {
    fn call(&self, _arg_count: usize, args: &[Value]) -> Value {
        if let Value::String(format) = &args[0] {
            let mut format = format.clone();
            args.iter()
                .skip(1)
                .for_each(|value| format = format.replacen("{}", &value.to_string(), 1));
            println!("{}", format);
        } else {
            panic!("println must have a format string.");
        }

        Value::None
    }
}

pub struct ConvertToNumber {}

impl NativeFunctionObject for ConvertToNumber {
    fn call(&self, _arg_count: usize, args: &[Value]) -> Value {
        use Value::*;
        match &args[0] {
            Number(a) => Number(*a),
            String(n) => {
                let number = n.parse::<f64>().expect("can convert this string to number");
                Number(number)
            }
            None => Number(0.0),
            _ => panic!("can not convert object to number"),
        }
    }
}

pub struct ConvertToString {}

impl NativeFunctionObject for ConvertToString {
    fn call(&self, _arg_count: usize, args: &[Value]) -> Value {
        use Value::*;
        match &args[0] {
            Number(a) => String(a.to_string()),
            String(n) => String(n.to_string()),
            None => String("none".to_string()),
            _ => panic!("can not convert object to string"),
        }
    }
}
