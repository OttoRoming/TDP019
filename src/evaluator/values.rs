use super::{Scope, builtins};
use crate::ast::{Block, Parameter};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
#[allow(unpredictable_function_pointer_comparisons)]
pub enum Function {
    Custom {
        scope: Rc<Scope>,
        parameters: Vec<Parameter>,
        body: Rc<Block>,
    },
    Builtin(builtins::Function),
}

#[derive(Debug, PartialEq, Clone)]
#[allow(unused)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Reference(Rc<RefCell<Value>>),
    String(String),
    List(Vec<Rc<RefCell<Value>>>),
    Function(Function),
    Void,
}

impl Value {
    #[allow(unused)]
    pub fn unwrap_int(&self) -> i64 {
        if let Value::Int(i) = self {
            *i
        } else {
            panic!("called `Value::unwrap_int()` on a non `Int` value")
        }
    }

    #[allow(unused)]
    pub fn unwrap_float(&self) -> f64 {
        if let Value::Float(f) = self {
            *f
        } else {
            panic!("called `Value::unwrap_float()` on a non `Int` value")
        }
    }

    pub fn unwrap_bool(&self) -> bool {
        if let Value::Bool(b) = self {
            *b
        } else {
            panic!("called `Value::unwrap_bool()` on a non `Bool` value")
        }
    }

    pub fn unwrap_str(&self) -> &str {
        if let Value::String(s) = self {
            s
        } else {
            panic!("called `Value::unwrap_str()` on a non `String` value")
        }
    }

    pub fn unwrap_reference(&self) -> Rc<RefCell<Value>> {
        if let Value::Reference(r) = self {
            Rc::clone(r)
        } else {
            panic!("called `Value::unwrap_reference()` on a non `Reference` value")
        }
    }

    pub fn unwrap_string(self) -> String {
        if let Value::String(s) = self {
            s
        } else {
            panic!("called `Value::unwrap_string()` on a non `String` value")
        }
    }

    pub fn unwrap_list(self) -> Vec<Rc<RefCell<Value>>> {
        if let Value::List(l) = self {
            l
        } else {
            panic!("called `Value::unwrap_list()` on a non `List` value")
        }
    }

    pub fn unwrap_function(self) -> Function {
        if let Value::Function(f) = self {
            f
        } else {
            panic!("called `Value::unwrap_function()` on a non `Function` value")
        }
    }
}

#[allow(unused)]
pub struct Return {
    pub value: Option<Rc<RefCell<Value>>>,
}

#[allow(unused)]
struct Exception {
    message: String,
}

#[allow(unused)]
enum ControlFlow {
    Return(Return),
    Exception(Exception),
}
