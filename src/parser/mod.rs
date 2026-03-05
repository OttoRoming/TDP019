use crate::{
    ast::{self, *},
    error::{self, Error},
    lexer::lex,
    token::{Token, Value},
    util::{Location, Region},
};

struct Parser {
    index: usize,
    tokens: Vec<Token>,
}

fn error(region: Region, message: String) -> Error {
    Error {
        message,
        level: error::Level::Parse,
        region: Some(region),
    }
}

impl Parser {
    fn peek(&self, ahead: usize) -> &Token {
        &self.tokens[self.index + ahead]
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn parse_statement(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub fn parse_program(&mut self) -> Result<ast::Statement, Error> {}

    pub fn new(source: &str) -> Result<Self, Error> {
        let tokens = lex(source)?;

        Ok(Self {
            index: 0,
            tokens: tokens,
        })
    }
}
