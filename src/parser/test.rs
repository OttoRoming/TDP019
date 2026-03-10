use super::parse;
use crate::ast::*;

#[test]
fn parse_parenthesized() {
    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Literal(
            LiteralExpression::Bool(true)
        ))]),
        parse("(((((true)))));")
    )
}

#[test]
fn parse_list_literal() {
    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Literal(
            LiteralExpression::List(vec![
                Expression::Literal(LiteralExpression::Int(1)),
                Expression::Literal(LiteralExpression::Int(2)),
                Expression::Literal(LiteralExpression::Int(3)),
            ])
        ))]),
        parse("[1, 2, 3];")
    )
}

#[test]
fn parse_logical_or() {
    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Bool(true)),
                operator: BinaryOperator::Equals,
                right: Expression::Literal(LiteralExpression::Bool(false)),
            }
        )))]),
        parse("true == false;")
    )
}

#[test]
fn parse_logical_and() {
    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Bool(true)),
                operator: BinaryOperator::And,
                right: Expression::Literal(LiteralExpression::Bool(false)),
            }
        )))]),
        parse("true && false;")
    )
}

#[test]
fn parse_logical_equality() {
    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Bool(true)),
                operator: BinaryOperator::Equals,
                right: Expression::Literal(LiteralExpression::Bool(false)),
            }
        )))]),
        parse("true == false;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Bool(true)),
                operator: BinaryOperator::NotEquals,
                right: Expression::Literal(LiteralExpression::Bool(false)),
            }
        )))]),
        parse("true != false;")
    )
}

#[test]
fn parse_comparison() {
    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::LessThan,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))]),
        parse("10 < 11;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::LessThanOrEqual,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))]),
        parse("10 <= 11;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::GreaterThan,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))]),
        parse("10 > 11;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::GreaterThanOrEqual,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))]),
        parse("10 >= 11;")
    );
}

#[test]
fn parse_additive() {
    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::Add,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))]),
        parse("10 + 11;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::Subtract,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))]),
        parse("10 - 11;")
    )
}

#[test]
fn parse_multiplicative() {
    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::Multiply,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))]),
        parse("10 * 11;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::Divide,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))]),
        parse("10 / 11;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Literal(LiteralExpression::Int(10)),
                operator: BinaryOperator::Modulo,
                right: Expression::Literal(LiteralExpression::Int(11))
            }
        )))]),
        parse("10 % 11;")
    )
}

#[test]
fn parse_function_call() {
    assert_eq!(
        Ok(vec![Statement::Expression(Expression::FunctionCall(
            Box::new(FunctionCallExpression {
                callee: Expression::Identifier(IdentifierExpression {
                    identifier: "foo".to_string()
                }),
                arguments: vec![
                    Expression::Literal(LiteralExpression::Int(1)),
                    Expression::Literal(LiteralExpression::Int(2)),
                    Expression::Literal(LiteralExpression::Int(3)),
                ]
            })
        ))]),
        parse("foo(1, 2, 3);")
    );
}

#[test]
fn parse_unary() {
    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Unary(Box::new(
            UnaryExpression {
                operator: UnaryOperator::Negate,
                right: Expression::Literal(LiteralExpression::Int(10))
            }
        )))]),
        parse("-10;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Unary(Box::new(
            UnaryExpression {
                operator: UnaryOperator::Not,
                right: Expression::Literal(LiteralExpression::Bool(true))
            }
        )))]),
        parse("!true;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Unary(Box::new(
            UnaryExpression {
                operator: UnaryOperator::Reference,
                right: Expression::Literal(LiteralExpression::Bool(true))
            }
        )))]),
        parse("&true;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Unary(Box::new(
            UnaryExpression {
                operator: UnaryOperator::Dereference,
                right: Expression::Literal(LiteralExpression::Bool(true))
            }
        )))]),
        parse("*true;")
    );
}

#[test]
fn parse_variable_declaration() {
    assert_eq!(
        Ok(vec![Statement::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: "x".to_string(),
                type_specifier: None,
                expression: Expression::Literal(LiteralExpression::Int(10))
            }
        )]),
        parse("var x = 10")
    )
}

#[test]
fn parse_variable_declaration_with_type() {
    assert_eq!(
        Ok(vec![Statement::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: "x".to_string(),
                type_specifier: Some(TypeSpecifier::Int),
                expression: Expression::Literal(LiteralExpression::Int(10))
            }
        )]),
        parse("var x: Int = 10")
    )
}

#[test]
fn parse_variable_declaration_with_generic_type() {
    assert_eq!(
        Ok(vec![Statement::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: "l".to_string(),
                type_specifier: Some(TypeSpecifier::List(Box::new(TypeSpecifier::Int))),
                expression: Expression::Literal(LiteralExpression::List(vec![
                    Expression::Literal(LiteralExpression::Int(1)),
                    Expression::Literal(LiteralExpression::Int(2)),
                    Expression::Literal(LiteralExpression::Int(3))
                ]))
            }
        )]),
        parse("var l: List<Int> = [1, 2, 3]")
    )
}

#[test]
fn parse_variable_declaration_with_extra_generic_type() {
    assert_eq!(
        Ok(vec![Statement::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: "l".to_string(),
                type_specifier: Some(TypeSpecifier::List(Box::new(TypeSpecifier::List(
                    Box::new(TypeSpecifier::Int)
                )))),
                expression: Expression::Literal(LiteralExpression::List(vec![
                    Expression::Literal(LiteralExpression::List(vec![
                        Expression::Literal(LiteralExpression::Int(1)),
                        Expression::Literal(LiteralExpression::Int(2)),
                        Expression::Literal(LiteralExpression::Int(3))
                    ])),
                    Expression::Literal(LiteralExpression::List(vec![
                        Expression::Literal(LiteralExpression::Int(4)),
                        Expression::Literal(LiteralExpression::Int(5)),
                        Expression::Literal(LiteralExpression::Int(6))
                    ])),
                    Expression::Literal(LiteralExpression::List(vec![
                        Expression::Literal(LiteralExpression::Int(7)),
                        Expression::Literal(LiteralExpression::Int(8)),
                        Expression::Literal(LiteralExpression::Int(9))
                    ]))
                ]))
            }
        )]),
        parse("var l: List<List<Int>> = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]")
    )
}
