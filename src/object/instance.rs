use super::r#struct::Struct;
use crate::value::Value;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Instance {
    r#struct: Rc<Struct>,
    fields: RefCell<HashMap<String, Value>>,
}

impl Instance {
    pub fn new(r#struct: Rc<Struct>) -> Self {
        Self {
            r#struct: Rc::clone(&r#struct),
            fields: RefCell::new(HashMap::new()),
        }
    }
}
