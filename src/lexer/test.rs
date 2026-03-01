use std::path::Path;

use super::lex;
use crate::{
    token::{Token, Value},
    util::{Location, Region},
};

#[test]
fn lex_string() {
    let path = Path::new("path");
    let tokens = lex("\"Hello, World\"", path).unwrap();
    assert_eq!(
        vec![
            Token {
                region: Region::new(Location::new(path, 1, 1), Location::new(path, 1, 15)),
                value: Value::String("Hello, World".to_string())
            },
            Token {
                region: Region::new(Location::new(path, 1, 15), Location::new(path, 1, 15)),
                value: Value::Eof
            }
        ],
        tokens
    );
}

#[test]
fn lex_ignore_whitespace() {
    let path = Path::new("path");
    let tokens = lex("\t  \n \n\n", path).unwrap();
    assert_eq!(
        vec![Token {
            region: Region::new(Location::new(path, 4, 1), Location::new(path, 4, 1)),
            value: Value::Eof
        }],
        tokens
    )
}

#[test]
fn lex_ignore_comments() {
    let path = Path::new("path");
    let tokens = lex("#!/usr/bin/env tdp019\n", path).unwrap();
    assert_eq!(
        vec![Token {
            region: Region::new(Location::new(path, 2, 1), Location::new(path, 2, 1)),
            value: Value::Eof
        }],
        tokens
    )
}

#[test]
fn lex_ignore_comments_without_newline() {
    let path = Path::new("path");
    let tokens = lex("#!/usr/bin/env tdp019", path).unwrap();
    assert_eq!(
        vec![Token {
            region: Region::new(Location::new(path, 1, 23), Location::new(path, 1, 23)),
            value: Value::Eof
        }],
        tokens
    )
}
