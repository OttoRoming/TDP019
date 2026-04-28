use super::Value;
use crate::{
    checker::{self, Type},
    evaluator::Scope,
};
use std::{cell::RefCell, rc::Rc};

// type definitions to avoid type complexity
// https://rust-lang.github.io/rust-clippy/rust-1.91.0/index.html#type_complexity
pub type Args = Vec<Rc<RefCell<Value>>>;
pub type Return = Rc<RefCell<Value>>;
pub type Function = fn(args: Args) -> Return;

pub fn checker_scopes() -> Vec<checker::Scope> {
    vec![
        checker::Scope::Identifier(checker::Identifier {
            identifier: "puts".to_string(),
            type_: Type::Function {
                parameters: vec![Type::String],
                return_type: Box::new(Some(Type::String)),
            },
        }),
        checker::Scope::Identifier(checker::Identifier {
            identifier: "range".to_string(),
            type_: Type::Function {
                parameters: vec![Type::Int],
                return_type: Box::new(Some(Type::List(Some(Box::new(Type::Int))))),
            },
        }),
    ]
}

pub fn evaluator_scopes() -> Rc<super::Scope> {
    Scope::new_builtin(
        Scope::new_builtin(Scope::new_block(None), "puts", puts),
        "range",
        range,
    )
}

pub fn puts(args: Args) -> Return {
    let msg = args.first().unwrap().borrow().clone().unwrap_string();
    println!("{}", msg);
    Rc::new(RefCell::new(Value::String(msg)))
}

pub fn range(args: Args) -> Return {
    let max = args.first().unwrap().borrow().unwrap_int();

    Rc::new(RefCell::new(Value::List(
        (0..max)
            .map(|i| Rc::new(RefCell::new(Value::Int(i))))
            .collect(),
    )))
}
