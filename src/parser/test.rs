use super::parse;
use crate::ast::*;
use crate::util::{Location, Region};

fn region(start: usize, end: usize) -> Region {
    Region::new(Location::new(1, start), Location::new(1, end))
}

fn expr(value: ExpressionValue, start: usize, end: usize) -> Expression {
    Expression {
        value,
        region: region(start, end),
    }
}

fn stmt(value: StatementValue, start: usize, end: usize) -> Statement {
    Statement {
        value,
        region: region(start, end),
    }
}

fn assert_parse(source: &str, expected: Vec<Statement>) {
    assert_eq!(Ok(expected), parse(source));
}

#[test]
fn parse_parenthesized() {
    assert_parse(
        "(((((true)))));",
        vec![stmt(
            StatementValue::Expression(expr(ExpressionValue::Bool(true), 1, 15)),
            1,
            16,
        )],
    )
}

#[test]
fn parse_list_literal() {
    assert_parse(
        "[1, 2, 3];",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::List(vec![
                    expr(ExpressionValue::Int(1), 2, 3),
                    expr(ExpressionValue::Int(2), 5, 6),
                    expr(ExpressionValue::Int(3), 8, 9),
                ]),
                1,
                10,
            )),
            1,
            11,
        )],
    )
}

#[test]
fn parse_logical_or() {
    assert_parse(
        "true == false;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: ExpressionValue::Bool(true),
                    operator: BinaryOperator::Equals,
                    right: ExpressionValue::Bool(false),
                })),
                1,
                14,
            )),
            1,
            15,
        )],
    )
}

#[test]
fn parse_logical_and() {
    assert_parse(
        "true && false;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: ExpressionValue::Bool(true),
                    operator: BinaryOperator::And,
                    right: ExpressionValue::Bool(false),
                })),
                1,
                14,
            )),
            1,
            15,
        )],
    )
}

#[test]
fn parse_logical_equality() {
    assert_parse(
        "true == false;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: ExpressionValue::Bool(true),
                    operator: BinaryOperator::Equals,
                    right: ExpressionValue::Bool(false),
                })),
                1,
                14,
            )),
            1,
            15,
        )],
    );

    assert_parse(
        "true != false;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: ExpressionValue::Bool(true),
                    operator: BinaryOperator::NotEquals,
                    right: ExpressionValue::Bool(false),
                })),
                1,
                14,
            )),
            1,
            15,
        )],
    )
}

#[test]
fn parse_comparison() {
    assert_parse(
        "10 < 11;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: ExpressionValue::Int(10),
                    operator: BinaryOperator::LessThan,
                    right: ExpressionValue::Int(11),
                })),
                1,
                8,
            )),
            1,
            9,
        )],
    );

    assert_parse(
        "10 <= 11;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: ExpressionValue::Int(10),
                    operator: BinaryOperator::LessThanOrEqual,
                    right: ExpressionValue::Int(11),
                })),
                1,
                9,
            )),
            1,
            10,
        )],
    );

    assert_parse(
        "10 > 11;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: ExpressionValue::Int(10),
                    operator: BinaryOperator::GreaterThan,
                    right: ExpressionValue::Int(11),
                })),
                1,
                8,
            )),
            1,
            9,
        )],
    );

    assert_parse(
        "10 >= 11;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: ExpressionValue::Int(10),
                    operator: BinaryOperator::GreaterThanOrEqual,
                    right: ExpressionValue::Int(11),
                })),
                1,
                9,
            )),
            1,
            10,
        )],
    );
}

#[test]
fn parse_additive() {
    assert_parse(
        "10 + 11;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: ExpressionValue::Int(10),
                    operator: BinaryOperator::Add,
                    right: ExpressionValue::Int(11),
                })),
                1,
                8,
            )),
            1,
            9,
        )],
    );

    assert_parse(
        "10 - 11;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: ExpressionValue::Int(10),
                    operator: BinaryOperator::Subtract,
                    right: ExpressionValue::Int(11),
                })),
                1,
                8,
            )),
            1,
            9,
        )],
    )
}

#[test]
fn parse_multiplicative() {
    assert_parse(
        "10 * 11;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: ExpressionValue::Int(10),
                    operator: BinaryOperator::Multiply,
                    right: ExpressionValue::Int(11),
                })),
                1,
                8,
            )),
            1,
            9,
        )],
    );

    assert_parse(
        "10 / 11;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: ExpressionValue::Int(10),
                    operator: BinaryOperator::Divide,
                    right: ExpressionValue::Int(11),
                })),
                1,
                8,
            )),
            1,
            9,
        )],
    );

    assert_parse(
        "10 % 11;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: ExpressionValue::Int(10),
                    operator: BinaryOperator::Modulo,
                    right: ExpressionValue::Int(11),
                })),
                1,
                8,
            )),
            1,
            9,
        )],
    )
}

#[test]
fn parse_function_call() {
    assert_parse(
        "foo(1, 2, 3);",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::FunctionCall(Box::new(FunctionCallExpression {
                    callee: ExpressionValue::Identifier(IdentifierExpression {
                        identifier: "foo".to_string(),
                    }),
                    arguments: vec![
                        expr(ExpressionValue::Int(1), 5, 6),
                        expr(ExpressionValue::Int(2), 8, 9),
                        expr(ExpressionValue::Int(3), 11, 12),
                    ],
                })),
                1,
                13,
            )),
            1,
            14,
        )],
    );
}

#[test]
fn parse_unary() {
    assert_parse(
        "-10;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Unary(Box::new(UnaryExpression {
                    operator: UnaryOperator::Negate,
                    right: ExpressionValue::Int(10),
                })),
                1,
                4,
            )),
            1,
            5,
        )],
    );

    assert_parse(
        "!true;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Unary(Box::new(UnaryExpression {
                    operator: UnaryOperator::Not,
                    right: ExpressionValue::Bool(true),
                })),
                1,
                6,
            )),
            1,
            7,
        )],
    );

    assert_parse(
        "&true;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Unary(Box::new(UnaryExpression {
                    operator: UnaryOperator::Reference,
                    right: ExpressionValue::Bool(true),
                })),
                1,
                6,
            )),
            1,
            7,
        )],
    );

    assert_parse(
        "*true;",
        vec![stmt(
            StatementValue::Expression(expr(
                ExpressionValue::Unary(Box::new(UnaryExpression {
                    operator: UnaryOperator::Dereference,
                    right: ExpressionValue::Bool(true),
                })),
                1,
                6,
            )),
            1,
            7,
        )],
    );
}

#[test]
fn parse_block() {
    assert_parse(
        "{}",
        vec![stmt(
            StatementValue::Block(Block { statements: vec![] }),
            1,
            3,
        )],
    )
}

#[test]
fn parse_variable_declaration() {
    assert_parse(
        "var x = 10",
        vec![stmt(
            StatementValue::VariableDeclaration(VariableDeclarationStatement {
                identifier: "x".to_string(),
                type_specifier: None,
                expression: expr(ExpressionValue::Int(10), 9, 11),
            }),
            1,
            11,
        )],
    )
}

#[test]
fn parse_variable_declaration_with_type() {
    assert_parse(
        "var x: Int = 10",
        vec![stmt(
            StatementValue::VariableDeclaration(VariableDeclarationStatement {
                identifier: "x".to_string(),
                type_specifier: Some(TypeSpecifier::Int),
                expression: expr(ExpressionValue::Int(10), 14, 16),
            }),
            1,
            16,
        )],
    )
}

#[test]
fn parse_variable_declaration_with_generic_type() {
    assert_parse(
        "var l: List<Int> = [1, 2, 3]",
        vec![stmt(
            StatementValue::VariableDeclaration(VariableDeclarationStatement {
                identifier: "l".to_string(),
                type_specifier: Some(TypeSpecifier::List(Box::new(TypeSpecifier::Int))),
                expression: expr(
                    ExpressionValue::List(vec![
                        expr(ExpressionValue::Int(1), 21, 22),
                        expr(ExpressionValue::Int(2), 24, 25),
                        expr(ExpressionValue::Int(3), 27, 28),
                    ]),
                    20,
                    29,
                ),
            }),
            1,
            29,
        )],
    )
}

#[test]
fn parse_variable_declaration_with_extra_generic_type() {
    assert_parse(
        "var l: List<List<Int>> = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]",
        vec![stmt(
            StatementValue::VariableDeclaration(VariableDeclarationStatement {
                identifier: "l".to_string(),
                type_specifier: Some(TypeSpecifier::List(Box::new(TypeSpecifier::List(
                    Box::new(TypeSpecifier::Int),
                )))),
                expression: expr(
                    ExpressionValue::List(vec![
                        expr(
                            ExpressionValue::List(vec![
                                expr(ExpressionValue::Int(1), 28, 29),
                                expr(ExpressionValue::Int(2), 31, 32),
                                expr(ExpressionValue::Int(3), 34, 35),
                            ]),
                            27,
                            36,
                        ),
                        expr(
                            ExpressionValue::List(vec![
                                expr(ExpressionValue::Int(4), 39, 40),
                                expr(ExpressionValue::Int(5), 42, 43),
                                expr(ExpressionValue::Int(6), 45, 46),
                            ]),
                            38,
                            47,
                        ),
                        expr(
                            ExpressionValue::List(vec![
                                expr(ExpressionValue::Int(7), 50, 51),
                                expr(ExpressionValue::Int(8), 53, 54),
                                expr(ExpressionValue::Int(9), 56, 57),
                            ]),
                            49,
                            58,
                        ),
                    ]),
                    26,
                    59,
                ),
            }),
            1,
            59,
        )],
    )
}

#[test]
fn parse_if() {
    assert_parse(
        "if true {}",
        vec![stmt(
            StatementValue::If(IfStatement {
                test: expr(ExpressionValue::Bool(true), 4, 8),
                block: Block { statements: vec![] },
                branch: None,
            }),
            1,
            11,
        )],
    )
}

#[test]
fn parse_if_elif() {
    assert_parse(
        "if false {} elif true {}",
        vec![stmt(
            StatementValue::If(IfStatement {
                test: expr(ExpressionValue::Bool(false), 4, 9),
                block: Block { statements: vec![] },
                branch: Some(IfBranch::Elif(ElifPart {
                    test: expr(ExpressionValue::Bool(true), 18, 22),
                    block: Block { statements: vec![] },
                    branch: Box::new(None),
                })),
            }),
            1,
            25,
        )],
    )
}

#[test]
fn parse_if_else() {
    assert_parse(
        "if false {} else {}",
        vec![stmt(
            StatementValue::If(IfStatement {
                test: expr(ExpressionValue::Bool(false), 4, 9),
                block: Block { statements: vec![] },
                branch: Some(IfBranch::Else(ElsePart {
                    block: Block { statements: vec![] },
                })),
            }),
            1,
            20,
        )],
    )
}

#[test]
fn parse_if_elif_else() {
    assert_parse(
        "if false {} elif false {} else {}",
        vec![stmt(
            StatementValue::If(IfStatement {
                test: expr(ExpressionValue::Bool(false), 4, 9),
                block: Block { statements: vec![] },
                branch: Some(IfBranch::Elif(ElifPart {
                    test: expr(ExpressionValue::Bool(false), 18, 23),
                    block: Block { statements: vec![] },
                    branch: Box::new(Some(IfBranch::Else(ElsePart {
                        block: Block { statements: vec![] },
                    }))),
                })),
            }),
            1,
            34,
        )],
    )
}
