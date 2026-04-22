use super::lex;
use crate::{
    token::{Token, Value},
    util::Region,
};
use pretty_assertions::assert_eq;

#[test]
fn string() {
    assert_eq!(
        Ok(vec![
            Token {
                region: Region::newi(1, 1, 1, 15),
                value: Value::String("Hello, World".to_string())
            },
            Token {
                region: Region::newi(1, 15, 1, 15),
                value: Value::Eof
            }
        ]),
        lex("\"Hello, World\"")
    );
}

#[test]
fn string_escape() {
    assert_eq!(
        Ok(vec![
            Token {
                value: Value::String("\u{7}\u{101}\u{33}\u{12}\n\r\t\u{11}\\\"".to_string()),
                region: Region::newi(1, 1, 1, 23),
            },
            Token {
                value: Value::Eof,
                region: Region::newi(1, 23, 1, 23),
            }
        ]),
        lex("\"\\a\\b\\e\\f\\n\\r\\t\\v\\\\\\\"\"")
    )
}

#[test]
fn ignore_whitespace() {
    assert_eq!(
        Ok(vec![Token {
            region: Region::newi(4, 1, 4, 1),
            value: Value::Eof
        }]),
        lex("\t  \n \n\n")
    )
}

#[test]
fn ignore_comments() {
    assert_eq!(
        Ok(vec![Token {
            region: Region::newi(2, 1, 2, 1),
            value: Value::Eof
        }]),
        lex("#!/usr/bin/env oeno\n")
    )
}

#[test]
fn ignore_comments_without_newline() {
    assert_eq!(
        Ok(vec![Token {
            region: Region::newi(1, 21, 1, 21),
            value: Value::Eof
        }]),
        lex("#!/usr/bin/env oeno")
    )
}
