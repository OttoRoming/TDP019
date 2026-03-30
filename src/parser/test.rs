use super::parse;
use crate::ast::*;
use crate::util::{Location, Region};

#[test]
fn parse_parenthesized() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Bool(true),
                region: Region::new(Location::new(1, 1), Location::new(1, 15)),
            }),
            region: Region::new(Location::new(1, 1), Location::new(1, 16)),
        }]),
        parse("(((((true)))));")
    )
}

#[test]
fn parse_list_literal() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::List(vec![
                    Expression {
                        value: ExpressionValue::Int(1),
                        region: Region::new(Location::new(1, 2), Location::new(1, 3)),
                    },
                    Expression {
                        value: ExpressionValue::Int(2),
                        region: Region::new(Location::new(1, 5), Location::new(1, 6)),
                    },
                    Expression {
                        value: ExpressionValue::Int(3),
                        region: Region::new(Location::new(1, 8), Location::new(1, 9)),
                    },
                ]),
                region: Region::new(Location::new(1, 1), Location::new(1, 10)),
            }),
            region: Region::new(Location::new(1, 1), Location::new(1, 11)),
        }]),
        parse("[1, 2, 3];")
    )
}

// #[test]
// fn parse_logical_or() {
//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Binary(Box::new(BinaryExpression {
//                     left: ExpressionValue::Bool(true),
//                     operator: BinaryOperator::Equals,
//                     right: ExpressionValue::Bool(false),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 14)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 15)),
//         }]),
//         parse("true == false;")
//     )
// }

// #[test]
// fn parse_logical_and() {
//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Binary(Box::new(BinaryExpression {
//                     left: ExpressionValue::Bool(true),
//                     operator: BinaryOperator::And,
//                     right: ExpressionValue::Bool(false),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 14)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 15)),
//         }]),
//         parse("true && false;")
//     )
// }

// #[test]
// fn parse_logical_equality() {
//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Binary(Box::new(BinaryExpression {
//                     left: ExpressionValue::Bool(true),
//                     operator: BinaryOperator::Equals,
//                     right: ExpressionValue::Bool(false),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 14)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 15)),
//         }]),
//         parse("true == false;")
//     );

//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Binary(Box::new(BinaryExpression {
//                     left: ExpressionValue::Bool(true),
//                     operator: BinaryOperator::NotEquals,
//                     right: ExpressionValue::Bool(false),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 14)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 15)),
//         }]),
//         parse("true != false;")
//     )
// }

// #[test]
// fn parse_comparison() {
//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Binary(Box::new(BinaryExpression {
//                     left: ExpressionValue::Int(10),
//                     operator: BinaryOperator::LessThan,
//                     right: ExpressionValue::Int(11),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 8)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 9)),
//         }]),
//         parse("10 < 11;")
//     );

//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Binary(Box::new(BinaryExpression {
//                     left: ExpressionValue::Int(10),
//                     operator: BinaryOperator::LessThanOrEqual,
//                     right: ExpressionValue::Int(11),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 9)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 10)),
//         }]),
//         parse("10 <= 11;")
//     );

//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Binary(Box::new(BinaryExpression {
//                     left: ExpressionValue::Int(10),
//                     operator: BinaryOperator::GreaterThan,
//                     right: ExpressionValue::Int(11),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 8)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 9)),
//         }]),
//         parse("10 > 11;")
//     );

//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Binary(Box::new(BinaryExpression {
//                     left: ExpressionValue::Int(10),
//                     operator: BinaryOperator::GreaterThanOrEqual,
//                     right: ExpressionValue::Int(11),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 9)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 10)),
//         }]),
//         parse("10 >= 11;")
//     );
// }

// #[test]
// fn parse_additive() {
//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Binary(Box::new(BinaryExpression {
//                     left: ExpressionValue::Int(10),
//                     operator: BinaryOperator::Add,
//                     right: ExpressionValue::Int(11),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 8)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 9)),
//         }]),
//         parse("10 + 11;")
//     );

//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Binary(Box::new(BinaryExpression {
//                     left: ExpressionValue::Int(10),
//                     operator: BinaryOperator::Subtract,
//                     right: ExpressionValue::Int(11),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 8)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 9)),
//         }]),
//         parse("10 - 11;")
//     )
// }

// #[test]
// fn parse_multiplicative() {
//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Binary(Box::new(BinaryExpression {
//                     left: ExpressionValue::Int(10),
//                     operator: BinaryOperator::Multiply,
//                     right: ExpressionValue::Int(11),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 8)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 9)),
//         }]),
//         parse("10 * 11;")
//     );

//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Binary(Box::new(BinaryExpression {
//                     left: ExpressionValue::Int(10),
//                     operator: BinaryOperator::Divide,
//                     right: ExpressionValue::Int(11),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 8)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 9)),
//         }]),
//         parse("10 / 11;")
//     );

//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Binary(Box::new(BinaryExpression {
//                     left: ExpressionValue::Int(10),
//                     operator: BinaryOperator::Modulo,
//                     right: ExpressionValue::Int(11),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 8)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 9)),
//         }]),
//         parse("10 % 11;")
//     )
// }

// #[test]
// fn parse_function_call() {
//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::FunctionCall(Box::new(FunctionCallExpression {
//                     callee: ExpressionValue::Identifier(IdentifierExpression {
//                         identifier: "foo".to_string(),
//                     }),
//                     arguments: vec![
//                         Expression {
//                             value: ExpressionValue::Int(1),
//                             region: Region::new(Location::new(1, 5), Location::new(1, 6)),
//                         },
//                         Expression {
//                             value: ExpressionValue::Int(2),
//                             region: Region::new(Location::new(1, 8), Location::new(1, 9)),
//                         },
//                         Expression {
//                             value: ExpressionValue::Int(3),
//                             region: Region::new(Location::new(1, 11), Location::new(1, 12)),
//                         },
//                     ],
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 13)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 14)),
//         }]),
//         parse("foo(1, 2, 3);")
//     );
// }

// #[test]
// fn parse_unary() {
//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Unary(Box::new(UnaryExpression {
//                     operator: UnaryOperator::Negate,
//                     right: ExpressionValue::Int(10),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 4)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 5)),
//         }]),
//         parse("-10;")
//     );

//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Unary(Box::new(UnaryExpression {
//                     operator: UnaryOperator::Not,
//                     right: ExpressionValue::Bool(true),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 6)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 7)),
//         }]),
//         parse("!true;")
//     );

//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Unary(Box::new(UnaryExpression {
//                     operator: UnaryOperator::Reference,
//                     right: ExpressionValue::Bool(true),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 6)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 7)),
//         }]),
//         parse("&true;")
//     );

//     assert_eq!(
//         Ok(vec![Statement {
//             value: StatementValue::Expression(Expression {
//                 value: ExpressionValue::Unary(Box::new(UnaryExpression {
//                     operator: UnaryOperator::Dereference,
//                     right: ExpressionValue::Bool(true),
//                 })),
//                 region: Region::new(Location::new(1, 1), Location::new(1, 6)),
//             }),
//             region: Region::new(Location::new(1, 1), Location::new(1, 7)),
//         }]),
//         parse("*true;")
//     );
// }

#[test]
fn parse_block() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Block(Block { statements: vec![] }),
            region: Region::new(Location::new(1, 1), Location::new(1, 3)),
        }]),
        parse("{}")
    )
}

#[test]
fn parse_variable_declaration() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::VariableDeclaration(VariableDeclarationStatement {
                identifier: "x".to_string(),
                type_specifier: None,
                expression: Expression {
                    value: ExpressionValue::Int(10),
                    region: Region::new(Location::new(1, 9), Location::new(1, 11)),
                },
            }),
            region: Region::new(Location::new(1, 1), Location::new(1, 11)),
        }]),
        parse("var x = 10")
    )
}

#[test]
fn parse_variable_declaration_with_type() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::VariableDeclaration(VariableDeclarationStatement {
                identifier: "x".to_string(),
                type_specifier: Some(TypeSpecifier::Int),
                expression: Expression {
                    value: ExpressionValue::Int(10),
                    region: Region::new(Location::new(1, 14), Location::new(1, 16)),
                },
            }),
            region: Region::new(Location::new(1, 1), Location::new(1, 16)),
        }]),
        parse("var x: Int = 10")
    )
}

#[test]
fn parse_variable_declaration_with_generic_type() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::VariableDeclaration(VariableDeclarationStatement {
                identifier: "l".to_string(),
                type_specifier: Some(TypeSpecifier::List(Box::new(TypeSpecifier::Int))),
                expression: Expression {
                    value: ExpressionValue::List(vec![
                        Expression {
                            value: ExpressionValue::Int(1),
                            region: Region::new(Location::new(1, 21), Location::new(1, 22)),
                        },
                        Expression {
                            value: ExpressionValue::Int(2),
                            region: Region::new(Location::new(1, 24), Location::new(1, 25)),
                        },
                        Expression {
                            value: ExpressionValue::Int(3),
                            region: Region::new(Location::new(1, 27), Location::new(1, 28)),
                        },
                    ]),
                    region: Region::new(Location::new(1, 20), Location::new(1, 29)),
                },
            }),
            region: Region::new(Location::new(1, 1), Location::new(1, 29)),
        }]),
        parse("var l: List<Int> = [1, 2, 3]")
    )
}

#[test]
fn parse_variable_declaration_with_extra_generic_type() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::VariableDeclaration(VariableDeclarationStatement {
                identifier: "l".to_string(),
                type_specifier: Some(TypeSpecifier::List(Box::new(TypeSpecifier::List(
                    Box::new(TypeSpecifier::Int,)
                )))),
                expression: Expression {
                    value: ExpressionValue::List(vec![
                        Expression {
                            value: ExpressionValue::List(vec![
                                Expression {
                                    value: ExpressionValue::Int(1),
                                    region: Region::new(Location::new(1, 28), Location::new(1, 29)),
                                },
                                Expression {
                                    value: ExpressionValue::Int(2),
                                    region: Region::new(Location::new(1, 31), Location::new(1, 32)),
                                },
                                Expression {
                                    value: ExpressionValue::Int(3),
                                    region: Region::new(Location::new(1, 34), Location::new(1, 35)),
                                },
                            ]),
                            region: Region::new(Location::new(1, 27), Location::new(1, 36)),
                        },
                        Expression {
                            value: ExpressionValue::List(vec![
                                Expression {
                                    value: ExpressionValue::Int(4),
                                    region: Region::new(Location::new(1, 39), Location::new(1, 40)),
                                },
                                Expression {
                                    value: ExpressionValue::Int(5),
                                    region: Region::new(Location::new(1, 42), Location::new(1, 43)),
                                },
                                Expression {
                                    value: ExpressionValue::Int(6),
                                    region: Region::new(Location::new(1, 45), Location::new(1, 46)),
                                },
                            ]),
                            region: Region::new(Location::new(1, 38), Location::new(1, 47)),
                        },
                        Expression {
                            value: ExpressionValue::List(vec![
                                Expression {
                                    value: ExpressionValue::Int(7),
                                    region: Region::new(Location::new(1, 50), Location::new(1, 51)),
                                },
                                Expression {
                                    value: ExpressionValue::Int(8),
                                    region: Region::new(Location::new(1, 53), Location::new(1, 54)),
                                },
                                Expression {
                                    value: ExpressionValue::Int(9),
                                    region: Region::new(Location::new(1, 56), Location::new(1, 57)),
                                },
                            ]),
                            region: Region::new(Location::new(1, 49), Location::new(1, 58)),
                        },
                    ]),
                    region: Region::new(Location::new(1, 26), Location::new(1, 59)),
                },
            }),
            region: Region::new(Location::new(1, 1), Location::new(1, 59)),
        }]),
        parse("var l: List<List<Int>> = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]")
    )
}

#[test]
fn parse_if() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::If(IfStatement {
                test: Expression {
                    value: ExpressionValue::Bool(true),
                    region: Region::new(Location::new(1, 4), Location::new(1, 8)),
                },
                block: Block { statements: vec![] },
                branch: None,
            }),
            region: Region::new(Location::new(1, 1), Location::new(1, 11)),
        }]),
        parse("if true {}")
    )
}

#[test]
fn parse_if_elif() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::If(IfStatement {
                test: Expression {
                    value: ExpressionValue::Bool(false),
                    region: Region::new(Location::new(1, 4), Location::new(1, 9)),
                },
                block: Block { statements: vec![] },
                branch: Some(IfBranch::Elif(ElifPart {
                    test: Expression {
                        value: ExpressionValue::Bool(true),
                        region: Region::new(Location::new(1, 18), Location::new(1, 22)),
                    },
                    block: Block { statements: vec![] },
                    branch: Box::new(None),
                })),
            }),
            region: Region::new(Location::new(1, 1), Location::new(1, 25)),
        }]),
        parse("if false {} elif true {}")
    )
}

#[test]
fn parse_if_else() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::If(IfStatement {
                test: Expression {
                    value: ExpressionValue::Bool(false),
                    region: Region::new(Location::new(1, 4), Location::new(1, 9)),
                },
                block: Block { statements: vec![] },
                branch: Some(IfBranch::Else(ElsePart {
                    block: Block { statements: vec![] },
                })),
            }),
            region: Region::new(Location::new(1, 1), Location::new(1, 20)),
        }]),
        parse("if false {} else {}")
    )
}

#[test]
fn parse_if_elif_else() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::If(IfStatement {
                test: Expression {
                    value: ExpressionValue::Bool(false),
                    region: Region::new(Location::new(1, 4), Location::new(1, 9)),
                },
                block: Block { statements: vec![] },
                branch: Some(IfBranch::Elif(ElifPart {
                    test: Expression {
                        value: ExpressionValue::Bool(false),
                        region: Region::new(Location::new(1, 18), Location::new(1, 23)),
                    },
                    block: Block { statements: vec![] },
                    branch: Box::new(Some(IfBranch::Else(ElsePart {
                        block: Block { statements: vec![] },
                    }))),
                })),
            }),
            region: Region::new(Location::new(1, 1), Location::new(1, 34)),
        }]),
        parse("if false {} elif false {} else {}")
    )
}
