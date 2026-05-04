use crate::{
    error::{self, Error},
    token::{Token, Value},
    util::{Location, Region},
};

#[cfg(test)]
mod test;

// https://compile7.org/special-characters/how-to-use-null-0-in-rust
const NULL_CHAR: char = '\0';

fn error(region: Region, message: String) -> Error {
    Error {
        message,
        level: error::Level::Lexer,
        region: Some(region),
    }
}

fn is_letter(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_number(c: char) -> bool {
    c.is_digit(10) || c == '.'
}

struct Lexer<'a> {
    location: Location,
    position: usize,
    read_position: usize,
    source: &'a str,
    char: char,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Self {
            location: Location::new(1, 0),
            position: 0,
            read_position: 0,
            source,
            char: NULL_CHAR,
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        self.char = self
            .source
            .chars()
            .nth(self.read_position)
            .unwrap_or(NULL_CHAR);

        self.position = self.read_position;
        self.read_position += 1;

        if self.char == '\n' {
            self.location.line += 1;
            self.location.column = 0;
        } else {
            self.location.column += 1;
        }
    }

    fn peek(&self) -> char {
        self.source
            .chars()
            .nth(self.read_position)
            .unwrap_or(NULL_CHAR)
    }

    fn skip_whitespace(&mut self) {
        while self.char.is_whitespace() {
            self.read_char();
        }
    }

    fn skip_comment(&mut self) {
        if self.char != '#' {
            return;
        }

        let start_line = self.location.line;

        while self.location.line == start_line && self.char != NULL_CHAR {
            self.read_char();
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        let mut previous = self.location.clone();

        loop {
            self.skip_whitespace();
            self.skip_comment();

            if self.location == previous {
                break;
            } else {
                previous = self.location.clone();
            }
        }
    }

    fn read_identifier_or_type_or_keyword(&mut self) -> Token {
        let start_location = self.location.clone();
        let start_position = self.position;

        while is_letter(self.char) || self.char.is_numeric() {
            self.read_char()
        }

        let end_position = self.position;
        let end_location = self.location.clone();

        let region = Region::new(start_location, end_location);
        let string = self.source[start_position..end_position].to_string();

        let value = match string.as_str() {
            "if" => Value::KeywordIf,
            "elif" => Value::KeywordElif,
            "else" => Value::KeywordElse,
            "fun" => Value::KeywordFun,
            "while" => Value::KeywordWhile,
            "each" => Value::KeywordEach,
            "return" => Value::KeywordReturn,
            "true" => Value::KeywordTrue,
            "false" => Value::KeywordFalse,
            "var" => Value::KeywordVar,
            "throw" => Value::KeywordThrow,
            "try" => Value::KeywordTry,
            "catch" => Value::KeywordCatch,
            "continue" => Value::KeywordContinue,
            "break" => Value::KeywordBreak,
            "Int" => Value::TypeInt,
            "Float" => Value::TypeFloat,
            "String" => Value::TypeString,
            "Bool" => Value::TypeBool,
            "List" => Value::TypeList,
            "Ref" => Value::TypeRef,
            _ => Value::Identifier(string),
        };

        Token { value, region }
    }

    fn read_number(&mut self) -> Result<Token, Error> {
        let start_location = self.location.clone();
        let start_position = self.position;

        while is_number(self.char) {
            self.read_char()
        }

        let end_position = self.position;
        let end_location = self.location.clone();

        let region = Region::new(start_location, end_location);
        let string = &self.source[start_position..end_position];

        let value = if let Ok(int) = string.parse::<i64>() {
            Value::Int(int)
        } else if let Ok(float) = string.parse::<f64>() {
            Value::Float(float)
        } else {
            return Err(error(region.clone(), "failed to parse number".to_string()));
        };

        Ok(Token { value, region })
    }

    fn read_string(&mut self) -> Result<Token, Error> {
        let start_location = self.location.clone();

        self.read_char();
        let mut contents = "".to_string();
        while self.char != '"' && self.char != '\0' {
            if self.char == '\\' {
                // Escape sequences taken from https://en.wikipedia.org/wiki/Escape_sequences_in_C
                // Correct assci value taken from https://www.ascii-code.com/
                dbg!(self.peek());
                let escaped_char = match self.peek() {
                    'a' => '\u{07}', // bell
                    'b' => '\u{08}', // backspace
                    'e' => '\u{1B}', // escape
                    'f' => '\u{0C}', // form feed
                    'n' => '\n',     // newline
                    'r' => '\r',     // carriage return
                    't' => '\t',     // horizontal tab
                    'v' => '\u{0B}', // vertical tab
                    '\\' => '\\',
                    '"' => '"',
                    _ => {
                        return Err(error(
                            Region::new(start_location, self.location.clone()),
                            format!("unknown string escape sequence (\\{})", self.peek()),
                        ));
                    }
                };

                contents.push(escaped_char);
                self.read_char();
                self.read_char();
            } else {
                contents.push(self.char);
                self.read_char();
            }
        }
        self.read_char();

        let end_location = self.location.clone();

        let region = Region::new(start_location, end_location);
        let value = Value::String(contents);

        Ok(Token { value, region })
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        self.skip_whitespace_and_comments();

        let mut length: usize = 2;

        let value = match (self.char, self.peek()) {
            ('&', '&') => Value::And,
            ('|', '|') => Value::Or,
            ('+', '=') => Value::AddAssign,
            ('-', '=') => Value::SubtractAssign,
            ('*', '=') => Value::MultiplyAssign,
            ('/', '=') => Value::DivideAssign,
            ('%', '=') => Value::ModAssign,
            ('!', '=') => Value::NotEquals,
            ('=', '=') => Value::DoubleEquals,
            ('<', '=') => Value::LessThanOrEqual,
            ('>', '=') => Value::GreaterThanOrEqual,
            ('&', '=') => Value::AndAssign,
            ('|', '=') => Value::OrAssign,
            ('+', '+') => Value::Increment,
            ('-', '-') => Value::Decrement,
            ('<', '-') => Value::Arrow,

            _ => {
                length = 1;

                match self.char {
                    '[' => Value::OpenBracket,
                    ']' => Value::CloseBracket,
                    '{' => Value::OpenBrace,
                    '}' => Value::CloseBrace,
                    '(' => Value::OpenParenthesis,
                    ')' => Value::CloseParenthesis,
                    '=' => Value::SingleEquals,
                    '+' => Value::Add,
                    '-' => Value::Subtract,
                    '*' => Value::Multiply,
                    '/' => Value::Divide,
                    '%' => Value::Mod,
                    '!' => Value::Not,
                    '<' => Value::LessThan,
                    '>' => Value::GreaterThan,
                    '&' => Value::Ampersand,
                    ';' => Value::Semicolon,
                    ':' => Value::Colon,
                    ',' => Value::Comma,
                    NULL_CHAR => Value::Eof,

                    _ => {
                        if is_letter(self.char) {
                            return Ok(self.read_identifier_or_type_or_keyword());
                        } else if is_number(self.char) {
                            return self.read_number();
                        } else if self.char == '"' {
                            return self.read_string();
                        } else {
                            return Err(error(
                                Region::new(self.location.clone(), self.location.clone() + 1),
                                format!("illegal character ({})", self.char),
                            ));
                        }
                    }
                }
            }
        };

        if value == Value::Eof {
            length = 0;
        }

        let start_location = self.location.clone();
        for _ in 0..length {
            self.read_char();
        }
        let end_location = self.location.clone();

        let region = Region::new(start_location, end_location);

        return Ok(Token { value, region });
    }
}

pub fn lex(source: &str) -> Result<Vec<Token>, Error> {
    let mut lexer = Lexer::new(source);

    let mut tokens: Vec<Token> = vec![];

    while tokens.last().map(|t| &t.value) != Some(&Value::Eof) {
        tokens.push(lexer.next_token()?);
    }

    Ok(tokens)
}
