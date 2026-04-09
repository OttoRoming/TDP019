use std::rc::Rc;

#[allow(dead_code)]
enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    List(Vec<Value>),
    Ref(Rc<Value>),
    Function {},
}

#[allow(dead_code)]
enum ControlFlow {}

// pub fn eval(source: &str) -> Result<> {

// }
