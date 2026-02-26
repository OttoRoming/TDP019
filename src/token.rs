use crate::util::Region;

#[derive(Debug)]
pub struct Token {
    value: Value,
    region: Region,
}

#[derive(Debug)]
pub enum Value {
    KeywordIf,     // if
    KeywordElif,   // elif
    KeywordElse,   // else
    KeywordFun,    // fun
    KeywordWhile,  // while
    KeywordEach,   // each
    KeywordNull,   // null
    KeywordReturn, // return

    OpenBracket,      // [
    CloseBracket,     // ]
    OpenBrace,        // {
    CloseBrace,       // }
    OpenParenthesis,  // (
    CloseParenthesis, // )
    LeftArrow,        // <-
    Equals,           // =
    Plus,             // +
    Minus,            // -
    Times,            // *
    Slash,            // /

    String(String), // "Hello, World"
    Int(i64),       // 67
    Float(f64),     // 67.67

    Identifier(String), // i
}
