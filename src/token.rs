use crate::util::Region;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub value: Value,
    pub region: Region,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    // keyword tokens
    KeywordIf,       // if
    KeywordElif,     // elif
    KeywordElse,     // else
    KeywordFun,      // fun
    KeywordWhile,    // while
    KeywordEach,     // each
    KeywordReturn,   // return
    KeywordTrue,     // true
    KeywordFalse,    // false
    KeywordVar,      // var
    KeywordThrow,    // throw
    KeywordTry,      // try
    KeywordCatch,    // catch
    KeywordContinue, // continue
    KeywordBreak,    // break

    TypeInt,    // Int
    TypeFloat,  // Float
    TypeString, // String
    TypeBool,   // Bool
    TypeList,   // List
    TypeRef,    // Ref

    // 2 char tokens
    And,                // &&
    Or,                 // ||
    AddAssign,          // +=
    SubtractAssign,     // -=
    MultiplyAssign,     // *=
    DivideAssign,       // /=
    ModAssign,          // %=
    NotEquals,          // !=
    DoubleEquals,       // ==
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=
    AndAssign,          // &=
    OrAssign,           // |=
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
    Ampersand,        // &
    Semicolon,        // ;
    Colon,            // :
    Comma,            // ,

    // Special tokens
    String(String),     // "Hello, World"
    Int(i64),           // 1337
    Float(f64),         // 3.14
    Identifier(String), // i

    Eof,
}
