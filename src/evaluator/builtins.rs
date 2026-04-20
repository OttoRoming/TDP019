use super::Value;
use std::{cell::RefCell, rc::Rc};

pub fn puts(args: Vec<Rc<RefCell<Value>>>) -> Rc<RefCell<Value>> {
    let msg = args.first().unwrap().borrow().clone().unwrap_string();
    println!("{}", msg);
    Rc::new(RefCell::new(Value::String(msg)))
}
