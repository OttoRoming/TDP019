use super::lex;
use crate::{
    token::{Token, Value},
    util::{Location, Region},
};

#[test]
fn lex_string() {
    let tokens = lex("\"Hello, World\"").unwrap();
    assert_eq!(
        vec![
            Token {
                region: Region::new(Location::new(1, 1), Location::new(1, 15)),
                value: Value::String("Hello, World".to_string())
            },
            Token {
                region: Region::new(Location::new(1, 15), Location::new(1, 15)),
                value: Value::Eof
            }
        ],
        tokens
    );
}

#[test]
fn lex_string_escape() {
    let tokens = lex("\"\\a\\b\\e\\f\\n\\r\\t\\v\\\\\\\"\"").unwrap();
    assert_eq!(
        vec![
            Token {
                value: Value::String("\u{7}\u{101}\u{33}\u{12}\n\r\t\u{11}\\\"".to_string()),
                region: Region {
                    start: Location { line: 1, column: 1 },
                    end: Location {
                        line: 1,
                        column: 23
                    }
                }
            },
            Token {
                value: Value::Eof,
                region: Region {
                    start: Location {
                        line: 1,
                        column: 23
                    },
                    end: Location {
                        line: 1,
                        column: 23
                    }
                }
            }
        ],
        tokens
    )
}

#[test]
fn lex_ignore_whitespace() {
    let tokens = lex("\t  \n \n\n").unwrap();
    assert_eq!(
        vec![Token {
            region: Region::new(Location::new(4, 1), Location::new(4, 1)),
            value: Value::Eof
        }],
        tokens
    )
}

#[test]
fn lex_ignore_comments() {
    let tokens = lex("#!/usr/bin/env tdp019\n").unwrap();
    assert_eq!(
        vec![Token {
            region: Region::new(Location::new(2, 1), Location::new(2, 1)),
            value: Value::Eof
        }],
        tokens
    )
}

#[test]
fn lex_ignore_comments_without_newline() {
    let tokens = lex("#!/usr/bin/env tdp019").unwrap();
    assert_eq!(
        vec![Token {
            region: Region::new(Location::new(1, 23), Location::new(1, 23)),
            value: Value::Eof
        }],
        tokens
    )
}
