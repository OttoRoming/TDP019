use crate::{
    ast::{self, *},
    error::{self, Error},
    lexer::lex,
    token::{Token, Value},
    util::{Location, Region},
};

#[cfg(test)]
mod test;

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

    fn matching(&self, expected: Value) -> bool {
        self.peek(0).value == expected
    }

    fn parse_literal(&mut self) -> Result<LiteralExpression, Error> {
        match &self.peek(0).value {
            Value::KeywordTrue => {
                self.advance();
                Ok(LiteralExpression::Bool(true))
            }
            Value::KeywordFalse => {
                self.advance();
                Ok(LiteralExpression::Bool(false))
            }
            _ => Err(error(
                self.peek(0).region,
                format!("expected literal, found {:?}", self.peek(0).value),
            )),
        }
    }

    fn parse_equality(&mut self) -> Result<Expression, Error> {
        let mut expression = Expression::Literal(self.parse_literal()?);

        while self.matching(Value::EqualsOperator) || self.matching(Value::NotEquals) {
            let operator = Some(match self.peek(0).value {
                Value::EqualsOperator => BinaryOperator::Equals,
                Value::NotEquals => BinaryOperator::NotEquals,
                _ => unreachable!(),
            });
            self.advance();

            let right = Some(Expression::Literal(self.parse_literal()?));

            expression = Expression::Binary(Box::new(BinaryExpression {
                left: expression,
                operator: operator.unwrap(),
                right: right.unwrap(),
            }));
        }

        Ok(expression)
    }

    fn parse_or(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_equality()?;

        while self.matching(Value::Or) {
            let operator = BinaryOperator::Or;
            self.advance();

            let right = self.parse_equality()?;

            expression = Expression::Binary(Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        Ok(self.parse_equality()?)
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        Ok(Statement::Expression(self.parse_expression()?))
    }

    pub fn parse_program(&mut self) -> Result<Statement, Error> {
        self.parse_statement()
    }

    pub fn new(source: &str) -> Result<Self, Error> {
        let tokens = lex(source)?;

        Ok(Self {
            index: 0,
            tokens: tokens,
        })
    }
}

pub fn parse(source: &str) -> Result<Statement, Error> {
    let mut parser = Parser::new(source)?;
    parser.parse_program()
}
