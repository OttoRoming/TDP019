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
        checker::Scope::Identifier(checker::Identifier {
            identifier: "random".to_string(),
            type_: Type::Function {
                parameters: vec![],
                return_type: Box::new(Some(Type::Float)),
            },
        }),
        checker::Scope::Identifier(checker::Identifier {
            identifier: "sleep".to_string(),
            type_: Type::Function {
                parameters: vec![Type::Int],
                return_type: Box::new(Some(Type::Int)),
            },
        }),
        checker::Scope::Identifier(checker::Identifier {
            identifier: "clear".to_string(),
            type_: Type::Function {
                parameters: vec![],
                return_type: Box::new(None),
            },
        }),
        checker::Scope::Identifier(checker::Identifier {
            identifier: "i_to_s".to_string(),
            type_: Type::Function {
                parameters: vec![Type::Int],
                return_type: Box::new(Some(Type::String)),
            },
        }),
    ]
}

pub fn evaluator_scopes() -> Rc<super::Scope> {
    Scope::new_builtin(
        Some(Scope::new_builtin(
            Some(Scope::new_builtin(
                Some(Scope::new_builtin(
                    Some(Scope::new_builtin(
                        Some(Scope::new_builtin(None, "puts", puts)),
                        "range",
                        range,
                    )),
                    "random",
                    random,
                )),
                "sleep",
                sleep,
            )),
            "clear",
            clear,
        )),
        "i_to_s",
        i_to_s,
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

pub fn random(_args: Args) -> Return {
    Rc::new(RefCell::new(Value::Float(fastrand::f64())))
}

pub fn sleep(args: Args) -> Return {
    let ms = args.first().unwrap().borrow().unwrap_int();
    std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    Rc::new(RefCell::new(Value::Int(ms)))
}

pub fn clear(_args: Args) -> Return {
    print!("{}[2J", 27 as char);

    Rc::new(RefCell::new(Value::Void))
}

pub fn i_to_s(args: Args) -> Return {
    let i = args.first().unwrap().borrow().unwrap_int();
    Rc::new(RefCell::new(Value::String(i.to_string())))
}
