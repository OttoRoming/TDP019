use crate::{
    error::{self, Error},
    token::{Token, Value},
    util::{Location, Region},
};
use std::path::Path;

#[cfg(test)]
mod test;

fn error<'a>(region: Region<'a>, message: String) -> Error<'a> {
    Error {
        message: message,
        level: error::Level::Lexer,
        region: Some(region),
    }
}

struct Lexer<'a> {
    location: Location<'a>,
    index: usize,
    source: Vec<char>,
}

impl<'a> Lexer<'a> {
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

    fn tokenize_string(&mut self) -> Result<Token<'a>, Error<'a>> {
        let start = self.location;
        self.advance(); // skip the first "

        let mut contents = String::new();
        while self.peek(0) != '"' {
            contents.push(self.peek(0));
            self.advance();
        }

        self.advance(); // skip the second "
        let end = self.location;

        Ok(Token {
            value: Value::String(contents),
            region: Region::new(start, end),
        })
    }

    fn tokenize_int_or_float(&mut self) -> Result<Token<'a>, Error<'a>> {
        let start = self.location;
        let mut content = String::new();

        while self.peek(0).is_digit(10) || self.peek(0) == '.' {
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

    fn tokenize_identifier_or_keyword(&mut self) -> Result<Token<'a>, Error<'a>> {
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
            _ => Value::Identifier(content),
        };

        Ok(Token { value, region })
    }

    fn tokenize(&mut self) -> Result<Token<'a>, Error<'a>> {
        let two_chars = format!("{}{}", self.peek(0), self.peek(1));
        let mut token_value = match two_chars.as_str() {
            "&&" => Some(Value::And),
            "||" => Some(Value::Or),
            "+=" => Some(Value::AddAssign),
            "-=" => Some(Value::SubtractAssign),
            "*=" => Some(Value::MultiplyAssign),
            "/=" => Some(Value::DivideAssign),
            "%=" => Some(Value::ModAssign),
            "==" => Some(Value::EqualsOperator),
            "<=" => Some(Value::LessThanOrEqual),
            ">=" => Some(Value::GreaterThanOrEqual),
            "++" => Some(Value::Increment),
            "--" => Some(Value::Decrement),
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
        } else if self.peek(0).is_digit(10) {
            self.tokenize_int_or_float()
        } else if self.peek(0).is_alphabetic() {
            self.tokenize_identifier_or_keyword()
        } else {
            Err(error(
                self.current_region(),
                format!("unxepceted character found ({})", self.peek(0)),
            ))
        }
    }

    fn run_analysis(&mut self) -> Result<Vec<Token<'a>>, Error<'a>> {
        let mut tokens: Vec<Token<'a>> = vec![];

        self.skip_whitespace();
        self.skip_comments();
        while !self.is_finished() {
            tokens.push(self.tokenize()?);
            self.skip_whitespace();
            self.skip_comments();
        }

        tokens.push(Token {
            value: Value::EOF,
            region: self.current_region(),
        });
        Ok(tokens)
    }

    fn current_region(&self) -> Region<'a> {
        Region::new(self.location, self.location)
    }

    pub fn new(filepath: &'a Path, source: &str) -> Self {
        Self {
            location: Location::new(filepath, 1, 1),
            index: 0,
            source: source.chars().collect(),
        }
    }
}

pub fn lex<'a>(source: &str, filepath: &'a Path) -> Result<Vec<Token<'a>>, Error<'a>> {
    let mut lexer: Lexer<'a> = Lexer::new(filepath, source);
    lexer.run_analysis()
}
