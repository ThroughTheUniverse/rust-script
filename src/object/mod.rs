pub mod bound_method;
pub mod closure;
pub mod function;
pub mod instance;
pub mod native_function;
pub mod r#struct;
pub mod upvalue;
use std::rc::Rc;

use crate::chunk::Chunk;

enum ObjectKind {
    BoundMethod,
    Class,
    Closure,
    Function,
    Instance,
    Native,
    String,
    Upvalue,
}

struct Object {
    kind: ObjectKind,
    is_marked: bool,
    next: Rc<Object>,
}

struct FunctionObject {
    object: Object,
    arity: usize,
    upvalue_count: usize,
    chunk: Chunk,
    name: Rc<StringObject>,
}

struct StringObject {
    object: Object,
    string: String,
    hash: usize,
}
