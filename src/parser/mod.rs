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
            let operator = match self.peek(0).value {
                Value::EqualsOperator => BinaryOperator::Equals,
                Value::NotEquals => BinaryOperator::NotEquals,
                _ => unreachable!(),
            };
            self.advance();

            let right = Expression::Literal(self.parse_literal()?);

            expression = Expression::Binary(Box::new(BinaryExpression {
                left: expression,
                operator: operator,
                right: right,
            }));
        }

        Ok(expression)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_equality()?;

        while self.matching(Value::And) {
            let operator = BinaryOperator::And;
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

    fn parse_logical_or(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_logical_and()?;

        while self.matching(Value::Or) {
            let operator = BinaryOperator::Or;
            self.advance();

            let right = self.parse_logical_and()?;

            expression = Expression::Binary(Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        Ok(self.parse_logical_or()?)
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, Error> {
        let expression = self.parse_expression()?;

        if self.peek(0).value != Value::Semicolon {
            return Err(error(
                self.peek(0).region,
                format!(
                    "expected semicolon after expression, found {:?}",
                    self.peek(0).value
                ),
            ));
        }

        Ok(Statement::Expression(expression))
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        self.parse_expression_statement()
    }

    pub fn parse_program(&mut self) -> Result<Vec<Statement>, Error> {
        Ok(vec![self.parse_statement()?])
    }

    pub fn new(source: &str) -> Result<Self, Error> {
        let tokens = lex(source)?;

        Ok(Self {
            index: 0,
            tokens: tokens,
        })
    }
}

pub fn parse(source: &str) -> Result<Vec<Statement>, Error> {
    let mut parser = Parser::new(source)?;
    parser.parse_program()
}
