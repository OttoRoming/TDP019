use std::rc::Rc;

enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    List(Vec<Value>),
    Ref(Rc<Value>),
    Function {},
}

enum ControlFlow {}

// pub fn eval(source: &str) -> Result<> {

// }
