use crate::util::Region;

#[derive(Debug)]
pub struct Token<'a> {
    value: Value,
    region: Region<'a>,
}

#[derive(Debug)]
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
    And,                 // &&
    Or,                  // ||
    AddAssign,           // +=
    SubtractAssign,      // -=
    MultiplyAssign,      // *=
    DivideAssign,        // /=
    ModAssign,           // %=
    EqualsOperator,      // ==
    LessThanOrEquals,    // <=
    GreaterThanOrEquals, // >=

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
    Mod,              // &
    Not,              // !
    LessThan,         // <
    GreaterThan,      // >

    String(String), // "Hello, World"
    Int(i64),       // 67
    Float(f64),     // 67.67

    Identifier(String), // i
}
