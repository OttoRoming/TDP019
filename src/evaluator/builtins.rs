use super::Value;

pub fn puts(args: Vec<Value>) -> Value {
    let msg = args.first().unwrap().clone().unwrap_string();
    println!("{}", msg);
    Value::String(msg)
}
