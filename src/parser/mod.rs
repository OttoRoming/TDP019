use crate::{
    ast::*,
    error::{self, Error},
    lexer::lex,
    token::{Token, Value},
    util::Region,
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

    fn expect(&self, expected_value: Value, at_msg: &str) -> Result<(), Error> {
        if self.peek(0).value != expected_value {
            Err(error(
                self.peek(0).region,
                format!(
                    "expected {:?} token at {}, found {:?}",
                    expected_value,
                    at_msg,
                    self.peek(0).value
                ),
            ))
        } else {
            Ok(())
        }
    }

    fn parse_type_specifier(&mut self) -> Result<TypeSpecifier, Error> {
        fn generic(parser: &mut Parser) -> Result<TypeSpecifier, Error> {
            parser.advance();
            parser.expect(Value::LessThan, "generic type")?;
            parser.advance();

            let type_specifier = parser.parse_type_specifier()?;

            parser.expect(Value::GreaterThan, "generic type")?;

            Ok(type_specifier)
        }

        let type_specifier = match self.peek(0).value {
            Value::TypeInt => TypeSpecifier::Int,
            Value::TypeFloat => TypeSpecifier::Float,
            Value::TypeBool => TypeSpecifier::Bool,
            Value::TypeList => TypeSpecifier::List(Box::new(generic(self)?)),
            Value::TypeRef => TypeSpecifier::Ref(Box::new(generic(self)?)),
            _ => {
                return Err(error(
                    self.peek(0).region,
                    format!("expected type specifier, found {:?}", self.peek(0).value),
                ));
            }
        };
        self.advance();

        Ok(type_specifier)
    }

    fn parse_comma_separated_expressions(&mut self) -> Result<Vec<Expression>, Error> {
        let mut expressions = vec![];

        loop {
            expressions.push(self.parse_expression()?);
            if self.peek(0).value != Value::Comma {
                break;
            }
            self.advance();
        }

        Ok(expressions)
    }

    fn parse_primary(&mut self) -> Result<Expression, Error> {
        match &self.peek(0).value {
            Value::KeywordTrue => {
                self.advance();
                Ok(Expression::Literal(LiteralExpression::Bool(true)))
            }
            Value::KeywordFalse => {
                self.advance();
                Ok(Expression::Literal(LiteralExpression::Bool(false)))
            }
            Value::KeywordNull => {
                self.advance();
                Ok(Expression::Literal(LiteralExpression::Null))
            }
            Value::Int(i) => {
                let literal = LiteralExpression::Int(*i);
                self.advance();
                Ok(Expression::Literal(literal))
            }
            Value::Float(f) => {
                let literal = LiteralExpression::Float(*f);
                self.advance();
                Ok(Expression::Literal(literal))
            }
            Value::String(s) => {
                let literal = LiteralExpression::String(s.clone());
                self.advance();
                Ok(Expression::Literal(literal))
            }
            Value::OpenBracket => {
                self.advance();
                let expressions = self.parse_comma_separated_expressions()?;
                self.expect(Value::CloseBracket, "end of list literal")?;
                self.advance();
                Ok(Expression::Literal(LiteralExpression::List(expressions)))
            }
            Value::OpenParenthesis => {
                self.advance();
                let expression = self.parse_expression()?;
                self.expect(Value::CloseParenthesis, "end of parenthesized expression")?;
                self.advance();
                Ok(expression)
            }
            Value::Identifier(id) => {
                let expression = Expression::Identifier(IdentifierExpression {
                    identifier: id.clone(),
                });
                self.advance();
                Ok(expression)
            }
            _ => Err(error(
                self.peek(0).region,
                format!(
                    "expected primary expression, found {:?}",
                    self.peek(0).value
                ),
            )),
        }
    }

    fn parse_call(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_primary()?;

        while self.peek(0).value == Value::OpenParenthesis {
            self.advance();

            let arguments = self.parse_comma_separated_expressions()?;

            self.expect(Value::CloseParenthesis, "end of argument list")?;
            self.advance();

            expression = Expression::FunctionCall(Box::new(FunctionCallExpression {
                callee: expression,
                arguments,
            }))
        }

        Ok(expression)
    }

    fn parse_unary(&mut self) -> Result<Expression, Error> {
        Ok(
            if let Some(operator) = match self.peek(0).value {
                Value::Not => Some(UnaryOperator::Not),
                Value::Subtract => Some(UnaryOperator::Negate),
                Value::Multiply => Some(UnaryOperator::Dereference),
                Value::Ampersand => Some(UnaryOperator::Reference),
                _ => None,
            } {
                self.advance();
                let right = self.parse_call()?;
                Expression::Unary(Box::new(UnaryExpression { operator, right }))
            } else {
                self.parse_call()?
            },
        )
    }

    fn parse_multiplicative(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_unary()?;

        loop {
            let operator = match self.peek(0).value {
                Value::Multiply => BinaryOperator::Multiply,
                Value::Divide => BinaryOperator::Divide,
                Value::Mod => BinaryOperator::Modulo,
                _ => break,
            };
            self.advance();

            let right = self.parse_unary()?;
            expression = Expression::Binary(Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn parse_addative(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_multiplicative()?;

        loop {
            let operator = match self.peek(0).value {
                Value::Add => BinaryOperator::Add,
                Value::Subtract => BinaryOperator::Subtract,
                _ => break,
            };
            self.advance();

            let right = self.parse_multiplicative()?;
            expression = Expression::Binary(Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn parse_comparison(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_addative()?;

        loop {
            let operator = match self.peek(0).value {
                Value::LessThan => BinaryOperator::LessThan,
                Value::LessThanOrEqual => BinaryOperator::LessThanOrEqual,
                Value::GreaterThan => BinaryOperator::GreaterThan,
                Value::GreaterThanOrEqual => BinaryOperator::GreaterThanOrEqual,
                _ => break,
            };
            self.advance();

            let right = self.parse_addative()?;
            expression = Expression::Binary(Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn parse_equality(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_comparison()?;

        loop {
            let operator = match self.peek(0).value {
                Value::DoubleEquals => BinaryOperator::Equals,
                Value::NotEquals => BinaryOperator::NotEquals,
                _ => break,
            };
            self.advance();

            let right = self.parse_comparison()?;
            expression = Expression::Binary(Box::new(BinaryExpression {
                left: expression,
                operator,
                right,
            }));
        }

        Ok(expression)
    }

    fn parse_logical_and(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_equality()?;

        while self.peek(0).value == Value::And {
            self.advance();

            let right = self.parse_equality()?;
            expression = Expression::Binary(Box::new(BinaryExpression {
                left: expression,
                operator: BinaryOperator::And,
                right,
            }));
        }

        Ok(expression)
    }

    fn parse_logical_or(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_logical_and()?;

        while self.peek(0).value == Value::Or {
            self.advance();

            let right = self.parse_logical_and()?;
            expression = Expression::Binary(Box::new(BinaryExpression {
                left: expression,
                operator: BinaryOperator::Or,
                right,
            }));
        }

        Ok(expression)
    }

    fn parse_update(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_logical_or()?;

        loop {
            let operator = match self.peek(0).value {
                Value::Increment => UpdateOperator::Increment,
                Value::Decrement => UpdateOperator::Decrement,
                _ => break,
            };
            self.advance();

            expression = Expression::Update(Box::new(UpdateExpression {
                updatee: expression,
                operator,
            }))
        }

        Ok(expression)
    }

    fn parse_assignment(&mut self) -> Result<Expression, Error> {
        let mut expression = self.parse_update()?;

        loop {
            let operator = match self.peek(0).value {
                Value::SingleEquals => AssignmentOperator::Equals,
                Value::AddAssign => AssignmentOperator::Add,
                Value::SubtractAssign => AssignmentOperator::Subtract,
                Value::MultiplyAssign => AssignmentOperator::Multiply,
                Value::DivideAssign => AssignmentOperator::Divide,
                Value::ModAssign => AssignmentOperator::Modulo,
                Value::AndAssign => AssignmentOperator::And,
                Value::OrAssign => AssignmentOperator::Or,
                _ => break,
            };
            self.advance();

            let right = self.parse_update()?;
            expression = Expression::Assign(Box::new(AssignmentExpression {
                assignee: expression,
                operator,
                right,
            }))
        }

        Ok(expression)
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.parse_assignment()
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, Error> {
        let expression = self.parse_expression()?;

        self.expect(Value::Semicolon, "end of expression statement")?;

        Ok(Statement::Expression(expression))
    }

    fn parse_variable_declaration(&mut self) -> Result<VariableDeclarationStatement, Error> {
        self.expect(Value::KeywordVar, "start of variable declaration statement")?;
        self.advance();

        let identifier = match &self.peek(0).value {
            Value::Identifier(id) => Ok(id.clone()),
            _ => Err(error(
                self.peek(0).region,
                format!(
                    "expected identifier for variable name, found {:?}",
                    self.peek(0).value
                ),
            )),
        }?;
        self.advance();

        let type_specifier = match self.peek(0).value {
            Value::Colon => {
                self.advance();
                Some(self.parse_type_specifier()?)
            }
            _ => None,
        };

        self.expect(Value::SingleEquals, "variable declaration")?;
        self.advance();

        let expression = self.parse_expression()?;

        Ok(VariableDeclarationStatement {
            identifier,
            type_specifier,
            expression,
        })
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        Ok(match self.peek(0).value {
            Value::KeywordVar => Statement::VariableDeclaration(self.parse_variable_declaration()?),
            _ => self.parse_expression_statement()?,
        })
    }

    pub fn parse_program(&mut self) -> Result<Vec<Statement>, Error> {
        Ok(vec![self.parse_statement()?])
    }

    pub fn new(source: &str) -> Result<Self, Error> {
        let tokens = lex(source)?;

        Ok(Self { index: 0, tokens })
    }
}

pub fn parse(source: &str) -> Result<Vec<Statement>, Error> {
    let mut parser = Parser::new(source)?;
    parser.parse_program()
}
