use super::parse;
use crate::ast::*;

#[test]
fn parse_parenthesized() {
    assert_eq!(
        vec![Statement::Expression(Expression::Literal(
            LiteralExpression::Bool(true)
        ))],
        parse("(((((true)))));").unwrap()
    )
}

#[test]
fn parse_logical_or() {
    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Bool(true)),
                operator: BinaryOperator::Equals,
                right: Expression::Literal(LiteralExpression::Bool(false)),
            }
        )))],
        parse("true == false;").unwrap()
    )
}

#[test]
fn parse_logical_and() {
    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Bool(true)),
                operator: BinaryOperator::And,
                right: Expression::Literal(LiteralExpression::Bool(false)),
            }
        )))],
        parse("true && false;").unwrap()
    )
}

#[test]
fn parse_logical_equality() {
    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Bool(true)),
                operator: BinaryOperator::Equals,
                right: Expression::Literal(LiteralExpression::Bool(false)),
            }
        )))],
        parse("true == false;").unwrap()
    );

    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Bool(true)),
                operator: BinaryOperator::NotEquals,
                right: Expression::Literal(LiteralExpression::Bool(false)),
            }
        )))],
        parse("true != false;").unwrap()
    )
}

#[test]
fn parse_comparison() {
    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::LessThan,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))],
        parse("10 < 11;").unwrap()
    );

    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::LessThanOrEqual,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))],
        parse("10 <= 11;").unwrap()
    );

    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::GreaterThan,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))],
        parse("10 > 11;").unwrap()
    );

    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::GreaterThanOrEqual,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))],
        parse("10 >= 11;").unwrap()
    );
}

#[test]
fn parse_additive() {
    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::Add,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))],
        parse("10 + 11;").unwrap()
    );

    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::Subtract,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))],
        parse("10 - 11;").unwrap()
    )
}

#[test]
fn parse_multiplicative() {
    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::Multiply,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))],
        parse("10 * 11;").unwrap()
    );

    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::Divide,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))],
        parse("10 / 11;").unwrap()
    );

    assert_eq!(
        vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::Modulo,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))],
        parse("10 % 11;").unwrap()
    )
}

#[test]
fn parse_unary() {
    assert_eq!(
        vec![Statement::Expression(Expression::Unary(Box::new(
            UnaryExpression {
                operator: UnaryOperator::Negate,
                right: Expression::Literal(LiteralExpression::Int(10))
            }
        )))],
        parse("-10;").unwrap()
    );

    assert_eq!(
        vec![Statement::Expression(Expression::Unary(Box::new(
            UnaryExpression {
                operator: UnaryOperator::Not,
                right: Expression::Literal(LiteralExpression::Bool(true))
            }
        )))],
        parse("!true;").unwrap()
    );

    assert_eq!(
        vec![Statement::Expression(Expression::Unary(Box::new(
            UnaryExpression {
                operator: UnaryOperator::Reference,
                right: Expression::Literal(LiteralExpression::Bool(true))
            }
        )))],
        parse("&true;").unwrap()
    );

    assert_eq!(
        vec![Statement::Expression(Expression::Unary(Box::new(
            UnaryExpression {
                operator: UnaryOperator::Dereference,
                right: Expression::Literal(LiteralExpression::Bool(true))
            }
        )))],
        parse("*true;").unwrap()
    );
}
