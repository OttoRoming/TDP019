use super::lex;
use crate::{
    token::{Token, Value},
    util::{Location, Region},
};
use pretty_assertions::assert_eq;

#[test]
fn lex_string() {
    assert_eq!(
        Ok(vec![
            Token {
                region: Region::new(Location::new(1, 1), Location::new(1, 15)),
                value: Value::String("Hello, World".to_string())
            },
            Token {
                region: Region::new(Location::new(1, 15), Location::new(1, 15)),
                value: Value::Eof
            }
        ]),
        lex("\"Hello, World\"")
    );
}

#[test]
fn lex_string_escape() {
    assert_eq!(
        Ok(vec![
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
        ]),
        lex("\"\\a\\b\\e\\f\\n\\r\\t\\v\\\\\\\"\"")
    )
}

#[test]
fn lex_ignore_whitespace() {
    assert_eq!(
        Ok(vec![Token {
            region: Region::new(Location::new(4, 1), Location::new(4, 1)),
            value: Value::Eof
        }]),
        lex("\t  \n \n\n")
    )
}

#[test]
fn lex_ignore_comments() {
    assert_eq!(
        Ok(vec![Token {
            region: Region::new(Location::new(2, 1), Location::new(2, 1)),
            value: Value::Eof
        }]),
        lex("#!/usr/bin/env oeno\n")
    )
}

#[test]
fn lex_ignore_comments_without_newline() {
    assert_eq!(
        Ok(vec![Token {
            region: Region::new(Location::new(1, 21), Location::new(1, 21)),
            value: Value::Eof
        }]),
        lex("#!/usr/bin/env oeno")
    )
}
