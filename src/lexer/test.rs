use std::path::Path;

use super::lex;
use crate::{
    token::{Token, Value},
    util::{Location, Region},
};

#[test]
fn lex_string() {
    let path = Path::new("");
    let tokens = lex("\"Hello, World\"", path).unwrap();
    assert_eq!(
        vec![Token {
            region: Region::new(Location::new(path, 1, 1), Location::new(path, 1, 15)),
            value: Value::String("Hello, World".to_string())
        }],
        tokens
    );
}
