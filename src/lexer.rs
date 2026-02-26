use crate::{
    error::{self, Error},
    fs::File,
    token::{self, Token},
    util::{Location, Region},
};
use std::{
    io::Read,
    iter::Peekable,
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

    fn tokenize(&mut self) -> Result<Token<'a>, Error<'a>> {
        self.peek(0)

        let t = match self.peek(0) {
            '[' => Some(token::Value::OpenBracket),
            ']' => Some(token::Value::CloseBracket),
            '{' => Some(token::Value::OpenBrace),
            '}' => Some(token::Value::CloseBrace),
            '(' => Some(token::Value::OpenParenthesis),
            ')' => Some(token::Value::CloseParenthesis),
            '=' => Some(token::Value::SingleEquals),
            '+' => Some(token::Value::Add),
            '-' => Some(token::Value::Subtract),
            '*' => Some(token::Value::Multiply),
            '/' => Some(token::Value::Divide),
            '!' => Some(token::Value::Not),
        };
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
