use crate::util::Region;

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub value: Value,
    pub region: Region<'a>,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    // keyword tokens
    KeywordIf,     // if
    KeywordElif,   // elif
    KeywordElse,   // else
    KeywordFun,    // fun
    KeywordWhile,  // while
    KeywordEach,   // each
    KeywordNull,   // null
    KeywordReturn, // return
    KeywordTrue,   // true
    KeywordFalse,  // false

    // 2 char tokens
    And,                // &&
    Or,                 // ||
    AddAssign,          // +=
    SubtractAssign,     // -=
    MultiplyAssign,     // *=
    DivideAssign,       // /=
    ModAssign,          // %=
    EqualsOperator,     // ==
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=
    Increment,          // ++
    Decrement,          // --

    // 1 char tokens
    OpenBracket,      // [
    CloseBracket,     // ]
    OpenBrace,        // {
    CloseBrace,       // }
    OpenParenthesis,  // (
    CloseParenthesis, // )
    SingleEquals,     // =
    Add,              // +
    Subtract,         // -
    Multiply,         // *
    Divide,           // /
    Mod,              // %
    Not,              // !
    LessThan,         // <
    GreaterThan,      // >

    // Special tokens
    String(String),     // "Hello, World"
    Int(i64),           // 1337
    Float(f64),         // 3.14
    Identifier(String), // i
}
