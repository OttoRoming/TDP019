use super::parse;
use crate::ast::*;
use crate::util::Region;
use pretty_assertions::assert_eq;

#[test]
fn parse_parenthesized() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Bool(true),
                region: Region::newi(1, 1, 1, 15)
            }),
            region: Region::newi(1, 1, 1, 16)
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
                        region: Region::newi(1, 2, 1, 3)
                    },
                    Expression {
                        value: ExpressionValue::Int(2),
                        region: Region::newi(1, 5, 1, 6)
                    },
                    Expression {
                        value: ExpressionValue::Int(3),
                        region: Region::newi(1, 8, 1, 9)
                    },
                ]),
                region: Region::newi(1, 1, 1, 10)
            }),
            region: Region::newi(1, 1, 1, 11)
        }]),
        parse("[1, 2, 3];")
    )
}

#[test]
fn parse_list_literal_empty() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::List(vec![]),
                region: Region::newi(1, 1, 1, 3)
            }),
            region: Region::newi(1, 1, 1, 4)
        }]),
        parse("[];")
    )
}

#[test]
fn parse_logical_or() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: Expression {
                        value: ExpressionValue::Bool(true),
                        region: Region::newi(1, 1, 1, 5)
                    },
                    operator: BinaryOperator::Or,
                    right: Expression {
                        value: ExpressionValue::Bool(false),
                        region: Region::newi(1, 9, 1, 14)
                    },
                }),),
                region: Region::newi(1, 1, 1, 14)
            },),
            region: Region::newi(1, 1, 1, 15)
        },],),
        parse("true || false;")
    );
}

#[test]
fn parse_logical_and() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: Expression {
                        value: ExpressionValue::Bool(true),
                        region: Region::newi(1, 1, 1, 5)
                    },
                    operator: BinaryOperator::And,
                    right: Expression {
                        value: ExpressionValue::Bool(false),
                        region: Region::newi(1, 9, 1, 14)
                    },
                }),),
                region: Region::newi(1, 1, 1, 14)
            },),
            region: Region::newi(1, 1, 1, 15)
        },],),
        parse("true && false;")
    )
}

#[test]
fn parse_logical_equality_equals() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: Expression {
                        value: ExpressionValue::Bool(true),
                        region: Region::newi(1, 1, 1, 5)
                    },
                    operator: BinaryOperator::Equals,
                    right: Expression {
                        value: ExpressionValue::Bool(false),
                        region: Region::newi(1, 9, 1, 14)
                    },
                }),),
                region: Region::newi(1, 1, 1, 14)
            },),
            region: Region::newi(1, 1, 1, 15)
        },],),
        parse("true == false;")
    );
}

#[test]
fn parse_logical_equality_not_equals() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: Expression {
                        value: ExpressionValue::Bool(true),
                        region: Region::newi(1, 1, 1, 5)
                    },
                    operator: BinaryOperator::NotEquals,
                    right: Expression {
                        value: ExpressionValue::Bool(false),
                        region: Region::newi(1, 9, 1, 14)
                    },
                }),),
                region: Region::newi(1, 1, 1, 14)
            },),
            region: Region::newi(1, 1, 1, 15)
        },],),
        parse("true != false;")
    )
}

#[test]
fn parse_comparison_less_than() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: Expression {
                        value: ExpressionValue::Int(10),
                        region: Region::newi(1, 1, 1, 3),
                    },
                    operator: BinaryOperator::LessThan,
                    right: Expression {
                        value: ExpressionValue::Int(11),
                        region: Region::newi(1, 6, 1, 8),
                    },
                })),
                region: Region::newi(1, 1, 1, 8),
            }),
            region: Region::newi(1, 1, 1, 9),
        }]),
        parse("10 < 11;")
    );
}

#[test]
fn parse_comparison_less_than_or_equal() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: Expression {
                        value: ExpressionValue::Int(10),
                        region: Region::newi(1, 1, 1, 3),
                    },
                    operator: BinaryOperator::LessThanOrEqual,
                    right: Expression {
                        value: ExpressionValue::Int(11),
                        region: Region::newi(1, 7, 1, 9),
                    },
                })),
                region: Region::newi(1, 1, 1, 9),
            }),
            region: Region::newi(1, 1, 1, 10),
        }]),
        parse("10 <= 11;")
    );
}

#[test]
fn parse_comparison_greater_than() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: Expression {
                        value: ExpressionValue::Int(10),
                        region: Region::newi(1, 1, 1, 3),
                    },
                    operator: BinaryOperator::GreaterThan,
                    right: Expression {
                        value: ExpressionValue::Int(11),
                        region: Region::newi(1, 6, 1, 8),
                    },
                })),
                region: Region::newi(1, 1, 1, 8),
            }),
            region: Region::newi(1, 1, 1, 9),
        }]),
        parse("10 > 11;")
    );
}

#[test]
fn parse_comparison_greater_than_or_equal() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: Expression {
                        value: ExpressionValue::Int(10),
                        region: Region::newi(1, 1, 1, 3),
                    },
                    operator: BinaryOperator::GreaterThanOrEqual,
                    right: Expression {
                        value: ExpressionValue::Int(11),
                        region: Region::newi(1, 7, 1, 9),
                    },
                })),
                region: Region::newi(1, 1, 1, 9),
            }),
            region: Region::newi(1, 1, 1, 10),
        }]),
        parse("10 >= 11;")
    );
}

#[test]
fn parse_additive_add() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: Expression {
                        value: ExpressionValue::Int(10),
                        region: Region::newi(1, 1, 1, 3)
                    },
                    operator: BinaryOperator::Add,
                    right: Expression {
                        value: ExpressionValue::Int(11),
                        region: Region::newi(1, 6, 1, 8)
                    },
                })),
                region: Region::newi(1, 1, 1, 8)
            }),
            region: Region::newi(1, 1, 1, 9)
        }]),
        parse("10 + 11;")
    );
}

#[test]
fn parse_additive_subtract() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: Expression {
                        value: ExpressionValue::Int(10),
                        region: Region::newi(1, 1, 1, 3)
                    },
                    operator: BinaryOperator::Subtract,
                    right: Expression {
                        value: ExpressionValue::Int(11),
                        region: Region::newi(1, 6, 1, 8)
                    },
                })),
                region: Region::newi(1, 1, 1, 8)
            }),
            region: Region::newi(1, 1, 1, 9)
        }]),
        parse("10 - 11;")
    )
}

#[test]
fn parse_multiplicative_multiply() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: Expression {
                        value: ExpressionValue::Int(10),
                        region: Region::newi(1, 1, 1, 3)
                    },
                    operator: BinaryOperator::Multiply,
                    right: Expression {
                        value: ExpressionValue::Int(11),
                        region: Region::newi(1, 6, 1, 8)
                    },
                })),
                region: Region::newi(1, 1, 1, 8)
            }),
            region: Region::newi(1, 1, 1, 9)
        }]),
        parse("10 * 11;")
    );
}

#[test]
fn parse_multiplicative_divide() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: Expression {
                        value: ExpressionValue::Int(10),
                        region: Region::newi(1, 1, 1, 3)
                    },
                    operator: BinaryOperator::Divide,
                    right: Expression {
                        value: ExpressionValue::Int(11),
                        region: Region::newi(1, 6, 1, 8)
                    },
                })),
                region: Region::newi(1, 1, 1, 8)
            }),
            region: Region::newi(1, 1, 1, 9)
        }]),
        parse("10 / 11;")
    );
}

#[test]
fn parse_multiplicative_modulo() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Binary(Box::new(BinaryExpression {
                    left: Expression {
                        value: ExpressionValue::Int(10),
                        region: Region::newi(1, 1, 1, 3)
                    },
                    operator: BinaryOperator::Modulo,
                    right: Expression {
                        value: ExpressionValue::Int(11),
                        region: Region::newi(1, 6, 1, 8)
                    },
                })),
                region: Region::newi(1, 1, 1, 8)
            }),
            region: Region::newi(1, 1, 1, 9)
        }]),
        parse("10 % 11;")
    )
}

#[test]
fn parse_function_call() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::FunctionCall(Box::new(FunctionCallExpression {
                    callee: Expression {
                        value: ExpressionValue::Identifier(IdentifierExpression {
                            identifier: "foo".to_string()
                        }),
                        region: Region::newi(1, 1, 1, 4),
                    },
                    arguments: vec![
                        Expression {
                            value: ExpressionValue::Int(1),
                            region: Region::newi(1, 5, 1, 6)
                        },
                        Expression {
                            value: ExpressionValue::Int(2),
                            region: Region::newi(1, 8, 1, 9)
                        },
                        Expression {
                            value: ExpressionValue::Int(3),
                            region: Region::newi(1, 11, 1, 12)
                        },
                    ],
                })),
                region: Region::newi(1, 1, 1, 13)
            }),
            region: Region::newi(1, 1, 1, 14)
        }]),
        parse("foo(1, 2, 3);")
    );
}

#[test]
fn parse_function_call_simple() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::FunctionCall(Box::new(FunctionCallExpression {
                    callee: Expression {
                        value: ExpressionValue::Identifier(IdentifierExpression {
                            identifier: "foo".to_string()
                        }),
                        region: Region::newi(1, 1, 1, 4),
                    },
                    arguments: vec![],
                })),
                region: Region::newi(1, 1, 1, 6)
            }),
            region: Region::newi(1, 1, 1, 7)
        }]),
        parse("foo();")
    );
}

#[test]
fn parse_unary_negate() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Unary(Box::new(UnaryExpression {
                    operator: UnaryOperator::Negate,
                    right: Expression {
                        value: ExpressionValue::Int(10),
                        region: Region::newi(1, 2, 1, 4)
                    },
                })),
                region: Region::newi(1, 1, 1, 4)
            }),
            region: Region::newi(1, 1, 1, 5)
        }]),
        parse("-10;")
    );
}

#[test]
fn parse_unary_not() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Unary(Box::new(UnaryExpression {
                    operator: UnaryOperator::Not,
                    right: Expression {
                        value: ExpressionValue::Bool(true),
                        region: Region::newi(1, 2, 1, 6)
                    },
                })),
                region: Region::newi(1, 1, 1, 6)
            }),
            region: Region::newi(1, 1, 1, 7)
        }]),
        parse("!true;")
    );
}

#[test]
fn parse_unary_reference() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Unary(Box::new(UnaryExpression {
                    operator: UnaryOperator::Reference,
                    right: Expression {
                        value: ExpressionValue::Bool(true),
                        region: Region::newi(1, 2, 1, 6)
                    },
                })),
                region: Region::newi(1, 1, 1, 6)
            }),
            region: Region::newi(1, 1, 1, 7)
        }]),
        parse("&true;")
    );
}

#[test]
fn parse_unary_dereference() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Expression(Expression {
                value: ExpressionValue::Unary(Box::new(UnaryExpression {
                    operator: UnaryOperator::Dereference,
                    right: Expression {
                        value: ExpressionValue::Bool(true),
                        region: Region::newi(1, 2, 1, 6)
                    },
                })),
                region: Region::newi(1, 1, 1, 6)
            }),
            region: Region::newi(1, 1, 1, 7)
        }]),
        parse("*true;")
    );
}

#[test]
fn parse_block() {
    assert_eq!(
        Ok(vec![Statement {
            value: StatementValue::Block(Block { statements: vec![] }),
            region: Region::newi(1, 1, 1, 3)
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
                    region: Region::newi(1, 9, 1, 11)
                },
            }),
            region: Region::newi(1, 1, 1, 11)
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
                    region: Region::newi(1, 14, 1, 16)
                },
            }),
            region: Region::newi(1, 1, 1, 16)
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
                            region: Region::newi(1, 21, 1, 22)
                        },
                        Expression {
                            value: ExpressionValue::Int(2),
                            region: Region::newi(1, 24, 1, 25)
                        },
                        Expression {
                            value: ExpressionValue::Int(3),
                            region: Region::newi(1, 27, 1, 28)
                        },
                    ]),
                    region: Region::newi(1, 20, 1, 29)
                },
            }),
            region: Region::newi(1, 1, 1, 29)
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
                                    region: Region::newi(1, 28, 1, 29)
                                },
                                Expression {
                                    value: ExpressionValue::Int(2),
                                    region: Region::newi(1, 31, 1, 32)
                                },
                                Expression {
                                    value: ExpressionValue::Int(3),
                                    region: Region::newi(1, 34, 1, 35)
                                },
                            ]),
                            region: Region::newi(1, 27, 1, 36)
                        },
                        Expression {
                            value: ExpressionValue::List(vec![
                                Expression {
                                    value: ExpressionValue::Int(4),
                                    region: Region::newi(1, 39, 1, 40)
                                },
                                Expression {
                                    value: ExpressionValue::Int(5),
                                    region: Region::newi(1, 42, 1, 43)
                                },
                                Expression {
                                    value: ExpressionValue::Int(6),
                                    region: Region::newi(1, 45, 1, 46)
                                },
                            ]),
                            region: Region::newi(1, 38, 1, 47)
                        },
                        Expression {
                            value: ExpressionValue::List(vec![
                                Expression {
                                    value: ExpressionValue::Int(7),
                                    region: Region::newi(1, 50, 1, 51)
                                },
                                Expression {
                                    value: ExpressionValue::Int(8),
                                    region: Region::newi(1, 53, 1, 54)
                                },
                                Expression {
                                    value: ExpressionValue::Int(9),
                                    region: Region::newi(1, 56, 1, 57)
                                },
                            ]),
                            region: Region::newi(1, 49, 1, 58)
                        },
                    ]),
                    region: Region::newi(1, 26, 1, 59)
                },
            }),
            region: Region::newi(1, 1, 1, 59)
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
                    region: Region::newi(1, 4, 1, 8),
                },
                block: Block { statements: vec![] },
                branch: None,
            }),
            region: Region::newi(1, 1, 1, 11),
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
                    region: Region::newi(1, 4, 1, 9),
                },
                block: Block { statements: vec![] },
                branch: Some(IfBranch::Elif(ElifPart {
                    test: Expression {
                        value: ExpressionValue::Bool(true),
                        region: Region::newi(1, 18, 1, 22),
                    },
                    block: Block { statements: vec![] },
                    branch: Box::new(None),
                })),
            }),
            region: Region::newi(1, 1, 1, 25),
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
                    region: Region::newi(1, 4, 1, 9),
                },
                block: Block { statements: vec![] },
                branch: Some(IfBranch::Else(ElsePart {
                    block: Block { statements: vec![] },
                })),
            }),
            region: Region::newi(1, 1, 1, 20),
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
                    region: Region::newi(1, 4, 1, 9),
                },
                block: Block { statements: vec![] },
                branch: Some(IfBranch::Elif(ElifPart {
                    test: Expression {
                        value: ExpressionValue::Bool(false),
                        region: Region::newi(1, 18, 1, 23),
                    },
                    block: Block { statements: vec![] },
                    branch: Box::new(Some(IfBranch::Else(ElsePart {
                        block: Block { statements: vec![] },
                    }))),
                })),
            }),
            region: Region::newi(1, 1, 1, 34),
        }]),
        parse("if false {} elif false {} else {}")
    )
}
