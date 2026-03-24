use crate::{
    ast::*,
    error::{self, Error},
    util::Region,
};

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Eq)]
#[allow(unused)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    List(Box<Type>),
    Ref(Box<Type>),
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },
}

impl From<TypeSpecifier> for Type {
    fn from(value: TypeSpecifier) -> Self {
        match value {
            TypeSpecifier::Int => Type::Int,
            TypeSpecifier::Float => Type::Float,
            TypeSpecifier::String => Type::String,
            TypeSpecifier::Bool => Type::Bool,
            TypeSpecifier::List(inner) => Type::List(Box::new((*inner).into())),
            TypeSpecifier::Ref(inner) => Type::Ref(Box::new((*inner).into())),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Identifier {
    identifier: String,
    type_: Type,
}

#[derive(Debug, PartialEq, Eq)]
enum Scope {
    Block,
    Identifier(Identifier),
}

#[derive(Debug, PartialEq, Eq)]
struct Checker {
    scopes: Vec<Scope>,
}

fn error(region: Region, msg: String) -> Error {
    Error {
        message: msg,
        level: error::Level::Check,
        region: Some(region),
    }
}

impl Checker {
    fn exit_block(&mut self) {
        while *self
            .scopes
            .last()
            .expect("type checker environment is empty")
            != Scope::Block
        {
            self.scopes.pop();
        }
        self.scopes.pop();
    }
    fn enter_block(&mut self) {
        self.scopes.push(Scope::Block);
    }

    fn check_binary(&mut self, binary: &BinaryExpression, region: &Region) -> Result<Type, Error> {
        let left = self.check_expression(&binary.left, region)?.ok_or(error(
            *region,
            "left expression in binary expression has unknown type".to_string(),
        ))?;

        let right = self.check_expression(&binary.right, region)?.ok_or(error(
            *region,
            "right expression in binary expression has unknown type".to_string(),
        ))?;

        if left != right {
            return Err(error(
                *region,
                format!("binary expression type mismatch ({:?}, {:?})", left, right),
            ));
        }

        match binary.operator {
            BinaryOperator::Add
            | BinaryOperator::Subtract
            | BinaryOperator::Multiply
            | BinaryOperator::Divide
            | BinaryOperator::Modulo => match left {
                Type::Int | Type::Float => Ok(left),
                _ => Err(error(
                    *region,
                    format!(
                        "{:?} is incompatible with operator {:?}",
                        left, binary.operator
                    ),
                )),
            },
            _ => todo!(),
        }
    }

    fn check_expression(
        &mut self,
        expression: &ExpressionValue,
        region: &Region,
    ) -> Result<Option<Type>, Error> {
        match &expression {
            ExpressionValue::Bool(_) => Ok(Some(Type::Bool)),
            ExpressionValue::Null => Ok(None),
            ExpressionValue::Binary(binary) => self.check_binary(binary, region).map(|t| Some(t)),
            _ => todo!(),
        }
    }

    fn check_statement(&mut self, statement: &Statement) -> Result<(), Error> {
        match &statement.value {
            StatementValue::Block(block) => self.check_block(block),
            StatementValue::Expression(expression) => self
                .check_expression(&expression.value, &expression.region)
                .map(|_| ()),
            _ => {
                todo!()
            }
        }
    }

    fn check_block(&mut self, block: &Block) -> Result<(), Error> {
        self.enter_block();
        for statement in block.statements.iter() {
            self.check_statement(statement)?;
        }
        self.exit_block();

        Ok(())
    }

    pub fn check(&mut self, program: &Vec<Statement>) -> Result<(), Error> {
        for statement in program {
            self.check_statement(statement)?;
        }
        Ok(())
    }

    pub fn new() -> Self {
        Checker { scopes: vec![] }
    }
}

pub fn check(program: &Vec<Statement>) -> Result<(), Error> {
    let mut checker = Checker::new();
    checker.check(program)?;
    dbg!(checker);
    Ok(())
}
