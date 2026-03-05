use super::parse;
use crate::ast::*;

#[test]
fn parse_logical_or() {
    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Bool(true)),
                operator: BinaryOperator::Equals,
                right: Expression::Literal(LiteralExpression::Bool(true)),
            }
        )))],
        parse("true == true").unwrap()
    )
}

#[test]
fn parse_logical_and() {
    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Bool(true)),
                operator: BinaryOperator::And,
                right: Expression::Literal(LiteralExpression::Bool(true)),
            }
        )))],
        parse("true && true").unwrap()
    )
}

#[test]
fn parse_logical_equality() {
    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Bool(true)),
                operator: BinaryOperator::Equals,
                right: Expression::Literal(LiteralExpression::Bool(true)),
            }
        )))],
        parse("true == true").unwrap()
    );

    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Bool(true)),
                operator: BinaryOperator::NotEquals,
                right: Expression::Literal(LiteralExpression::Bool(true)),
            }
        )))],
        parse("true != true").unwrap()
    )
}
