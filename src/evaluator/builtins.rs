use super::Value;
use std::{cell::RefCell, rc::Rc};

// type definitions to avoid type complexity
// https://rust-lang.github.io/rust-clippy/rust-1.91.0/index.html#type_complexity
pub type Args = Vec<Rc<RefCell<Value>>>;
pub type Return = Rc<RefCell<Value>>;
pub type Function = fn(args: Args) -> Return;

pub fn puts(args: Args) -> Return {
    let msg = args.first().unwrap().borrow().clone().unwrap_string();
    println!("{}", msg);
    Rc::new(RefCell::new(Value::String(msg)))
}
