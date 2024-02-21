use super::r#struct::Struct;
use crate::value::Value;
use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

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

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} instance", self.r#struct)
    }
}
