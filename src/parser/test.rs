use super::parse;
use crate::ast::*;

#[test]
fn parse_parenthesized() {
    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Bool(true))]),
        parse("(((((true)))));")
    )
}

#[test]
fn parse_list_literal() {
    assert_eq!(
        Ok(vec![Statement::Expression(Expression::List(vec![
            Expression::Int(1),
            Expression::Int(2),
            Expression::Int(3),
        ]))]),
        parse("[1, 2, 3];")
    )
}

#[test]
fn parse_logical_or() {
    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Bool(true),
                operator: BinaryOperator::Equals,
                right: Expression::Bool(false),
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
                left: Expression::Bool(true),
                operator: BinaryOperator::And,
                right: Expression::Bool(false),
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
                left: Expression::Bool(true),
                operator: BinaryOperator::Equals,
                right: Expression::Bool(false),
            }
        )))]),
        parse("true == false;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Bool(true),
                operator: BinaryOperator::NotEquals,
                right: Expression::Bool(false),
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
                left: Expression::Int(10),
                operator: BinaryOperator::LessThan,
                right: Expression::Int(11)
            }
        )))]),
        parse("10 < 11;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Int(10),
                operator: BinaryOperator::LessThanOrEqual,
                right: Expression::Int(11)
            }
        )))]),
        parse("10 <= 11;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Int(10),
                operator: BinaryOperator::GreaterThan,
                right: Expression::Int(11)
            }
        )))]),
        parse("10 > 11;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Int(10),
                operator: BinaryOperator::GreaterThanOrEqual,
                right: Expression::Int(11)
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
                left: Expression::Int(10),
                operator: BinaryOperator::Add,
                right: Expression::Int(11)
            }
        )))]),
        parse("10 + 11;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Int(10),
                operator: BinaryOperator::Subtract,
                right: Expression::Int(11)
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
                left: Expression::Int(10),
                operator: BinaryOperator::Multiply,
                right: Expression::Int(11)
            }
        )))]),
        parse("10 * 11;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Int(10),
                operator: BinaryOperator::Divide,
                right: Expression::Int(11)
            }
        )))]),
        parse("10 / 11;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Binary(Box::new(
            BinaryExpression {
                left: Expression::Int(10),
                operator: BinaryOperator::Modulo,
                right: Expression::Int(11)
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
                arguments: vec![Expression::Int(1), Expression::Int(2), Expression::Int(3),]
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
                right: Expression::Int(10)
            }
        )))]),
        parse("-10;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Unary(Box::new(
            UnaryExpression {
                operator: UnaryOperator::Not,
                right: Expression::Bool(true)
            }
        )))]),
        parse("!true;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Unary(Box::new(
            UnaryExpression {
                operator: UnaryOperator::Reference,
                right: Expression::Bool(true)
            }
        )))]),
        parse("&true;")
    );

    assert_eq!(
        Ok(vec![Statement::Expression(Expression::Unary(Box::new(
            UnaryExpression {
                operator: UnaryOperator::Dereference,
                right: Expression::Bool(true)
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
                expression: Expression::Int(10)
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
                expression: Expression::Int(10)
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
                expression: Expression::List(vec![
                    Expression::Int(1),
                    Expression::Int(2),
                    Expression::Int(3)
                ])
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
                expression: Expression::List(vec![
                    Expression::List(vec![
                        Expression::Int(1),
                        Expression::Int(2),
                        Expression::Int(3)
                    ]),
                    Expression::List(vec![
                        Expression::Int(4),
                        Expression::Int(5),
                        Expression::Int(6)
                    ]),
                    Expression::List(vec![
                        Expression::Int(7),
                        Expression::Int(8),
                        Expression::Int(9)
                    ])
                ])
            }
        )]),
        parse("var l: List<List<Int>> = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]")
    )
}
