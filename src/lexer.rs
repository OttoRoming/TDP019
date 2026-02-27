use crate::{
    error::{self, Error},
    fs::File,
    token::{Token, Value},
    util::{Location, Region},
};
use std::{
    io::Read,
    iter::Peekable,
    ops::AddAssign,
    path::{Path, PathBuf},
    str::Chars,
};

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

    fn tokenize_string(&mut self) -> Result<Token<'a>, Error<'a>> {
        let mut contents = String::new();

        let start = self.location;
        while self.peek(1) != '"' {
            contents.push(self.peek(0));
        }
        let end = self.location;

        Ok(Token {
            value: Value::String(contents),
            region: Region::new(start, end),
        })
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
            let end = self.location;
            self.advance();
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
            _ => None,
        };
        if let Some(value) = token_value {
            let region = self.current_region();
            self.advance();
            return Ok(Token { value, region });
        }

        match self.peek(0) {
            '"' => self.tokenize_string(),
            _ => Err(error(
                self.current_region(),
                format!("unxepceted character found ({})", self.peek(0)),
            )),
        }
    }

    fn run_analysis(&mut self) -> Result<Vec<Token<'a>>, Error<'a>> {
        let mut tokens: Vec<Token<'a>> = vec![];

        while !self.is_finished() {
            tokens.push(self.tokenize()?);
        }

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

pub fn lex<'a>(filepath: &'a Path) -> Result<Vec<Token<'a>>, Error<'a>> {
    let mut file = File::open(&filepath)?;

    let mut source = String::new();
    file.read_to_string(&mut source)?;

    let mut lexer: Lexer<'a> = Lexer::new(filepath, &source);
    lexer.run_analysis()
}
