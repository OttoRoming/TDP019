use crate::{
    error::{self, Error},
    token::{Token, Value},
    util::{Location, Region},
};

#[cfg(test)]
mod test;

fn error(region: Region, message: String) -> Error {
    Error {
        message,
        level: error::Level::Lexer,
        region: Some(region),
    }
}

struct Lexer {
    location: Location,
    index: usize,
    source: Vec<char>,
}

impl Lexer {
    fn current_region(&self) -> Region {
        Region::new(self.location, self.location)
    }

    fn advance(&mut self) {
        if self.peek(0) == '\n' {
            self.location.line += 1;
            self.location.column = 1;
        } else {
            self.location.column += 1;
        }

        self.index += 1;
    }

    fn peek(&self, ahead: usize) -> char {
        match self.source.get(self.index + ahead) {
            Some(char) => *char,
            None => ' ',
        }
    }

    fn is_finished(&self) -> bool {
        self.index >= self.source.len()
    }

    fn skip_whitespace(&mut self) {
        while self.peek(0).is_whitespace() && !self.is_finished() {
            self.advance();
        }
    }

    fn skip_comments(&mut self) {
        while self.peek(0) == '#' {
            while self.peek(0) != '\n' && !self.is_finished() {
                self.advance();
            }
            self.advance();
        }
    }

    fn tokenize_string(&mut self) -> Result<Token, Error> {
        let start = self.location;
        self.advance(); // skip the first "

        let mut contents = String::new();
        while self.peek(0) != '"' {
            if self.peek(0) == '\\' {
                // Escape sequences taken from https://en.wikipedia.org/wiki/Escape_sequences_in_C
                // Correct assci value taken from https://www.asciitable.com/
                let escaped_char = match self.peek(1) {
                    'a' => '\u{7}',   // bell
                    'b' => '\u{101}', // backspace
                    'e' => '\u{33}',  // escape
                    'f' => '\u{12}',  // form feed
                    'n' => '\n',      // newline
                    'r' => '\r',      // carrige return
                    't' => '\t',      // horizontal tab
                    'v' => '\u{11}',  // vertical tab
                    '\\' => '\\',
                    '"' => '"',
                    _ => {
                        return Err(error(
                            Region::new(start, self.location),
                            format!("unknown string escape sequence (\\{})", self.peek(1)),
                        ));
                    }
                };

                contents.push(escaped_char);
                self.advance();
                self.advance();
            } else {
                contents.push(self.peek(0));
                self.advance();
            }
        }

        self.advance(); // skip the second "
        let end = self.location;

        Ok(Token {
            value: Value::String(contents),
            region: Region::new(start, end),
        })
    }

    fn tokenize_int_or_float(&mut self) -> Result<Token, Error> {
        let start = self.location;
        let mut content = String::new();

        while self.peek(0).is_ascii_digit() || self.peek(0) == '.' {
            content.push(self.peek(0));
            self.advance();
        }

        let end = self.location;
        let region = Region::new(start, end);

        let is_float = content.contains('.');
        let value = if is_float {
            Value::Float(
                content
                    .parse::<f64>()
                    .map_err(|e| error(region, format!("failed to parse float token ({})", e)))?,
            )
        } else {
            Value::Int(
                content
                    .parse::<i64>()
                    .map_err(|e| error(region, format!("failed to parse int token ({})", e)))?,
            )
        };

        Ok(Token { value, region })
    }

    fn tokenize_mutlichar(&mut self) -> Result<Token, Error> {
        let start = self.location;

        let mut content = String::new();
        while self.peek(0).is_alphanumeric() {
            content.push(self.peek(0));
            self.advance();
        }

        let end = self.location;
        let region = Region::new(start, end);

        let value = match content.as_str() {
            "if" => Value::KeywordIf,
            "elif" => Value::KeywordElif,
            "else" => Value::KeywordElse,
            "fun" => Value::KeywordFun,
            "while" => Value::KeywordWhile,
            "each" => Value::KeywordEach,
            "null" => Value::KeywordNull,
            "return" => Value::KeywordReturn,
            "true" => Value::KeywordTrue,
            "false" => Value::KeywordFalse,
            "var" => Value::KeywordVar,
            "Int" => Value::TypeInt,
            "Float" => Value::TypeFloat,
            "String" => Value::TypeString,
            "Bool" => Value::TypeBool,
            "List" => Value::TypeList,
            "Ref" => Value::TypeRef,
            _ => Value::Identifier(content),
        };

        Ok(Token { value, region })
    }

    fn tokenize(&mut self) -> Result<Token, Error> {
        let two_chars = (self.peek(0), self.peek(1));
        let mut token_value = match two_chars {
            ('&', '&') => Some(Value::And),
            ('|', '|') => Some(Value::Or),
            ('+', '=') => Some(Value::AddAssign),
            ('-', '=') => Some(Value::SubtractAssign),
            ('*', '=') => Some(Value::MultiplyAssign),
            ('/', '=') => Some(Value::DivideAssign),
            ('%', '=') => Some(Value::ModAssign),
            ('!', '=') => Some(Value::NotEquals),
            ('=', '=') => Some(Value::DoubleEquals),
            ('<', '=') => Some(Value::LessThanOrEqual),
            ('>', '=') => Some(Value::GreaterThanOrEqual),
            ('&', '=') => Some(Value::AndAssign),
            ('|', '=') => Some(Value::OrAssign),
            ('+', '+') => Some(Value::Increment),
            ('-', '-') => Some(Value::Decrement),
            _ => None,
        };
        if let Some(value) = token_value {
            let start = self.location;
            self.advance();
            self.advance();
            let end = self.location;
            let region = Region::new(start, end);

            return Ok(Token { value, region });
        }

        token_value = match self.peek(0) {
            '[' => Some(Value::OpenBracket),
            ']' => Some(Value::CloseBracket),
            '{' => Some(Value::OpenBrace),
            '}' => Some(Value::CloseBrace),
            '(' => Some(Value::OpenParenthesis),
            ')' => Some(Value::CloseParenthesis),
            '=' => Some(Value::SingleEquals),
            '+' => Some(Value::Add),
            '-' => Some(Value::Subtract),
            '*' => Some(Value::Multiply),
            '/' => Some(Value::Divide),
            '%' => Some(Value::Mod),
            '!' => Some(Value::Not),
            '<' => Some(Value::LessThan),
            '>' => Some(Value::GreaterThan),
            '&' => Some(Value::Ampersand),
            ';' => Some(Value::Semicolon),
            ':' => Some(Value::Colon),
            ',' => Some(Value::Comma),
            _ => None,
        };
        if let Some(value) = token_value {
            let start = self.location;
            self.advance();
            let end = self.location;
            let region = Region::new(start, end);
            return Ok(Token { value, region });
        }

        if self.peek(0) == '"' {
            self.tokenize_string()
        } else if self.peek(0).is_ascii_digit() {
            self.tokenize_int_or_float()
        } else if self.peek(0).is_alphabetic() {
            self.tokenize_mutlichar()
        } else {
            Err(error(
                self.current_region(),
                format!("unxepceted character found ({})", self.peek(0)),
            ))
        }
    }

    fn run_analysis(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = vec![];

        self.skip_whitespace();
        self.skip_comments();
        while !self.is_finished() {
            tokens.push(self.tokenize()?);
            self.skip_whitespace();
            self.skip_comments();
        }

        tokens.push(Token {
            value: Value::Eof,
            region: self.current_region(),
        });
        Ok(tokens)
    }

    pub fn new(source: &str) -> Self {
        Self {
            location: Location::new(1, 1),
            index: 0,
            source: source.chars().collect(),
        }
    }
}

pub fn lex(source: &str) -> Result<Vec<Token>, Error> {
    let mut lexer = Lexer::new(source);
    lexer.run_analysis()
}
