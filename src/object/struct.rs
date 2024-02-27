use std::{cell::RefCell, collections::HashMap, fmt::Display, rc::Rc};

use super::{closure::Closure, function::Function};

pub struct StructObject {
    pub name: String,
    pub methods: RefCell<HashMap<String, Rc<Function>>>,
    pub init: RefCell<Option<Rc<Function>>>,
}

impl StructObject {
    pub fn new(name: String) -> Self {
        Self {
            name,
            methods: RefCell::new(HashMap::new()),
            init: RefCell::new(None),
        }
    }
}

impl Display for StructObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
