use crate::{
    ast::*,
    error::{self, Error},
    util::Region,
};

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Eq, Clone)]
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
        return_type: Box<Option<Type>>,
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
    pub identifier: String,
    pub type_: Type,
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

impl<'a> Checker {
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
    fn declare_identifier(&mut self, identifier: Identifier) {
        self.scopes.push(Scope::Identifier(identifier))
    }
    fn get_identifier(&'a mut self, id: &str) -> Option<&'a Type> {
        for scope in self.scopes.iter().rev() {
            if let Scope::Identifier(scope_id) = scope {
                if scope_id.identifier == id {
                    return Some(&scope_id.type_);
                }
            }
        }

        None
    }

    fn check_binary(&mut self, binary: &BinaryExpression, region: &Region) -> Result<Type, Error> {
        let left = self.check_expression(&binary.left)?.ok_or(error(
            region.clone(),
            "left expression in binary expression has unknown type".to_string(),
        ))?;

        let right = self.check_expression(&binary.right)?.ok_or(error(
            region.clone(),
            "right expression in binary expression has unknown type".to_string(),
        ))?;

        if left != right {
            return Err(error(
                region.clone(),
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
                    region.clone(),
                    format!(
                        "{:?} is incompatible with operator {:?}",
                        left, binary.operator
                    ),
                )),
            },
            _ => todo!(),
        }
    }

    fn check_function_call(
        &mut self,
        call: &FunctionCallExpression,
        region: &Region,
    ) -> Result<Option<Type>, Error> {
        let callee_type = self.check_expression(&call.callee)?.ok_or(error(
            region.clone(),
            "attempted to call null type".to_string(),
        ))?;

        if let Type::Function {
            parameters,
            return_type,
        } = callee_type
        {
            if call.arguments.len() != parameters.len() {
                return Err(error(
                    region.clone(),
                    format!(
                        "wrong number of arguments in function call, (expected: {}, got: {})",
                        parameters.len(),
                        call.arguments.len()
                    ),
                ));
            }

            for (parameter, argument) in parameters.iter().zip(call.arguments.iter()) {
                let argument_type = self.check_expression(argument)?;
                if let Some(argument_type) = argument_type {
                    if *parameter != argument_type {
                        return Err(error(
                            region.clone(),
                            format!(
                                "argument type mismatch in function call (expected: {:?}, got: {:?})",
                                parameter, argument_type
                            ),
                        ));
                    }
                }
            }

            Ok(*return_type)
        } else {
            Err(error(
                region.clone(),
                format!("attempted to call non-function type: {:?}", callee_type),
            ))
        }
    }

    fn check_expression(&mut self, expression: &Expression) -> Result<Option<Type>, Error> {
        match &expression.value {
            ExpressionValue::Bool(_) => Ok(Some(Type::Bool)),
            ExpressionValue::Null => Ok(None),
            ExpressionValue::Binary(binary) => {
                self.check_binary(binary, &expression.region).map(Some)
            }
            ExpressionValue::FunctionCall(call) => {
                self.check_function_call(call, &expression.region)
            }
            _ => todo!(),
        }
    }

    fn get_varible_declaration_type(
        &mut self,
        var: &VariableDeclarationStatement,
        region: &Region,
    ) -> Result<Type, Error> {
        let expression_type_check = self.check_expression(&var.expression)?;

        if let Some(specifier) = var.type_specifier.as_ref().map(|s| Type::from(s.clone())) {
            // Variable declarations with specified type
            if let Some(expression) = expression_type_check
                && specifier != expression
            {
                return Err(error(
                    region.clone(),
                    format!(
                        "variable declaration type mismatch (specified: {:?}; got: {:?})",
                        specifier, expression
                    ),
                ));
            }

            Ok(specifier)
        } else {
            // Infered type
            if let Some(expression) = expression_type_check {
                Ok(expression)
            } else {
                Err(error(
                    region.clone(),
                    "could not infer type for variable declaration, hint: add a type specifier"
                        .to_string(),
                ))
            }
        }
    }

    fn check_variable_declaration(
        &mut self,
        var: &VariableDeclarationStatement,
        region: &Region,
    ) -> Result<(), Error> {
        let type_ = self.get_varible_declaration_type(var, region)?;

        self.declare_identifier(Identifier {
            identifier: var.identifier.clone(),
            type_,
        });

        Ok(())
    }

    fn check_statement(&mut self, statement: &Statement) -> Result<(), Error> {
        match &statement.value {
            StatementValue::Block(block) => self.check_block(block),
            StatementValue::VariableDeclaration(var) => {
                self.check_variable_declaration(var, &statement.region)
            }
            StatementValue::Expression(expression) => self.check_expression(expression).map(|_| ()),
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
