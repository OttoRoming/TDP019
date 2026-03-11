use super::check;
use crate::parser::parse;

#[test]
fn checker() {
    check(&parse("{}").unwrap()).unwrap()
}
