use crate::{
    ast::*,
    error::{self, Error},
    util::Region,
};

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(unused)]
enum Type {
    Int,
    Float,
    Bool,
    String,
    List(Option<Box<Type>>),
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
            TypeSpecifier::List(inner) => Type::List(Some(Box::new((*inner).into()))),
            TypeSpecifier::Ref(inner) => Type::Ref(Box::new((*inner).into())),
        }
    }
}

impl Type {
    fn is_matching(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::List(self_inner), Type::List(other_inner)) => {
                self_inner.is_none() || other_inner.is_none() || self_inner == other_inner
            }
            _ => self == other,
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
    function_bodies: Vec<Option<Type>>,
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
            // pop the identifiers inside the block
            self.scopes.pop();
        }
        // pop the block
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
            if let Scope::Identifier(scope_id) = scope
                && scope_id.identifier == id
            {
                return Some(&scope_id.type_);
            }
        }

        None
    }

    fn enter_function_body(&mut self, fun: Option<Type>) {
        self.function_bodies.push(fun);
    }
    fn exit_function_body(&mut self) {
        self.function_bodies.pop();
    }
    fn get_current_return_type(&'a mut self) -> Option<&'a Option<Type>> {
        self.function_bodies.last()
    }

    fn check_binary(&mut self, binary: &BinaryExpression, region: &Region) -> Result<Type, Error> {
        let left = self.check_expression(&binary.left)?.ok_or(error(
            region.clone(),
            "left expression in binary expression has void type".to_string(),
        ))?;

        let right = self.check_expression(&binary.right)?.ok_or(error(
            region.clone(),
            "right expression in binary expression has void type".to_string(),
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
        let callee_type = self.check_expression(&call.callee)?;

        if let Some(Type::Function {
            parameters,
            return_type,
        }) = callee_type
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
                if let Some(argument_type) = argument_type
                    && *parameter != argument_type
                {
                    return Err(error(
                        region.clone(),
                        format!(
                            "argument type mismatch in function call (expected: {:?}, got: {:?})",
                            parameter, argument_type
                        ),
                    ));
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

    fn check_assign(
        &mut self,
        assign: &AssignmentExpression,
        region: &Region,
    ) -> Result<Option<Type>, Error> {
        let assignee_type = self.check_expression(&assign.assignee)?;
        let right_type = self.check_expression(&assign.right)?;

        if assignee_type != right_type {
            return Err(error(
                region.clone(),
                format!(
                    "assign type mismatch, {:?} and {:?}",
                    assignee_type, right_type
                ),
            ));
        }

        let is_operator_compatible = match assign.operator {
            AssignmentOperator::And | AssignmentOperator::Or => right_type == Some(Type::Bool),
            AssignmentOperator::Add
            | AssignmentOperator::Divide
            | AssignmentOperator::Modulo
            | AssignmentOperator::Multiply
            | AssignmentOperator::Subtract => {
                right_type == Some(Type::Int) || right_type == Some(Type::Float)
            }
            AssignmentOperator::Equals => true,
        };

        if !is_operator_compatible {
            return Err(error(
                region.clone(),
                format!(
                    "assignment operator {:?} is incompatible with type {:?}",
                    assign.operator, right_type
                ),
            ));
        }

        Ok(right_type)
    }

    fn check_update(&mut self, update: &UpdateExpression, region: &Region) -> Result<Type, Error> {
        let updatee_type = self.check_expression(&update.updatee)?.ok_or(error(
            region.clone(),
            "can not update void type".to_string(),
        ))?;

        if updatee_type != Type::Int && updatee_type != Type::Float {
            return Err(error(
                region.clone(),
                format!("cannot update type {:?}", updatee_type),
            ));
        }

        Ok(updatee_type)
    }

    fn check_unary(&mut self, unary: &UnaryExpression, region: &Region) -> Result<Type, Error> {
        let right_type = self.check_expression(&unary.right)?.ok_or(error(
            region.clone(),
            "can not perform unary operation on void type".to_string(),
        ))?;

        let is_compatible = match unary.operator {
            UnaryOperator::Negate => right_type == Type::Int || right_type == Type::Float,
            UnaryOperator::Not => right_type == Type::Bool,
            UnaryOperator::Dereference | UnaryOperator::Reference => true,
        };

        if !is_compatible {
            return Err(error(
                region.clone(),
                format!(
                    "unary operator {:?} is incompatible with type {:?}",
                    unary.operator, right_type
                ),
            ));
        }

        Ok(right_type)
    }

    fn check_identifier(
        &mut self,
        identifier: &IdentifierExpression,
        region: &Region,
    ) -> Result<Type, Error> {
        self.get_identifier(&identifier.identifier)
            .ok_or(error(
                region.clone(),
                format!("undeclared identifier \"{}\"", &identifier.identifier),
            ))
            .cloned()
    }

    fn check_index(
        &mut self,
        index: &IndexExpression,
        region: &Region,
    ) -> Result<Option<Type>, Error> {
        let collection_type = self.check_expression(&index.collection)?;

        if let Some(Type::List(inner_type)) = collection_type {
            let index_type = self.check_expression(&index.index)?;
            if index_type != Some(Type::Int) {
                return Err(error(
                    region.clone(),
                    format!(
                        "tried to index into collection withh non int type, {:?}",
                        index_type
                    ),
                ));
            }

            Ok(inner_type.map(|t| *t))
        } else {
            Err(error(
                region.clone(),
                format!("tried to index into non list type, {:?}", collection_type),
            ))
        }
    }

    fn check_list(&mut self, list: &[Expression], region: &Region) -> Result<Type, Error> {
        if list.is_empty() {
            return Ok(Type::List(None));
        }

        let inner_type = self.check_expression(list.first().unwrap())?.ok_or(error(
            list.first().unwrap().region.clone(),
            "list can not include void type".to_string(),
        ))?;

        for expression in list.iter().skip(1) {
            let expression_type = self.check_expression(expression)?.ok_or(error(
                list.first().unwrap().region.clone(),
                "list can not include void type".to_string(),
            ))?;
            if inner_type != expression_type {
                return Err(error(
                    region.clone(),
                    format!(
                        "type mismatch in list, list includes types {:?} and {:?}",
                        inner_type, expression_type
                    ),
                ));
            }
        }

        Ok(Type::List(Some(Box::new(inner_type))))
    }

    fn check_expression(&mut self, expression: &Expression) -> Result<Option<Type>, Error> {
        match &expression.value {
            ExpressionValue::Bool(_) => Ok(Some(Type::Bool)),
            ExpressionValue::String(_) => Ok(Some(Type::String)),
            ExpressionValue::Int(_) => Ok(Some(Type::Int)),
            ExpressionValue::Float(_) => Ok(Some(Type::Float)),
            ExpressionValue::Binary(binary) => {
                self.check_binary(binary, &expression.region).map(Some)
            }
            ExpressionValue::FunctionCall(call) => {
                self.check_function_call(call, &expression.region)
            }
            ExpressionValue::Assign(assign) => self.check_assign(assign, &expression.region),
            ExpressionValue::Update(update) => {
                self.check_update(update, &expression.region).map(Some)
            }
            ExpressionValue::Unary(unary) => self.check_unary(unary, &expression.region).map(Some),
            ExpressionValue::Identifier(id) => {
                self.check_identifier(id, &expression.region).map(Some)
            }
            ExpressionValue::Index(index) => self.check_index(index, &expression.region),
            ExpressionValue::List(list) => self.check_list(list, &expression.region).map(Some),
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

    fn check_function_declaration(
        &mut self,
        function: &FunctionDeclarationStatement,
    ) -> Result<(), Error> {
        self.enter_block();

        for parameter in &function.parameters {
            self.declare_identifier(Identifier {
                identifier: parameter.identifier.clone(),
                type_: parameter.type_specifier.clone().into(),
            });
        }

        let return_type: Option<Type> = function.return_type.clone().map(|t| t.into());
        self.enter_function_body(return_type.clone());
        self.check_block(&function.block)?;
        self.exit_function_body();

        self.exit_block();

        self.declare_identifier(Identifier {
            identifier: function.identifier.clone(),
            type_: Type::Function {
                parameters: function
                    .parameters
                    .iter()
                    .map(|t| t.type_specifier.clone().into())
                    .collect(),
                return_type: Box::new(return_type),
            },
        });

        Ok(())
    }

    fn check_if_branch(&mut self, if_branch: &IfBranch) -> Result<(), Error> {
        match if_branch {
            IfBranch::Elif(elif) => {
                let test_type = self.check_expression(&elif.test)?;
                if test_type != Some(Type::Bool) {
                    return Err(error(
                        elif.test.region.clone(),
                        format!("elif branch can only check booleans, found {:?}", test_type),
                    ));
                };

                self.check_block(&elif.block)?;
            }
            IfBranch::Else(else_part) => {
                self.check_block(&else_part.block)?;
            }
        }

        Ok(())
    }

    fn check_if_statement(&mut self, if_statement: &IfStatement) -> Result<(), Error> {
        let test_type = self.check_expression(&if_statement.test)?;
        if test_type != Some(Type::Bool) {
            return Err(error(
                if_statement.test.region.clone(),
                format!(
                    "if statements can only check booleans, found {:?}",
                    test_type
                ),
            ));
        };

        self.check_block(&if_statement.block)?;

        if let Some(branch) = &if_statement.branch {
            self.check_if_branch(branch)?;
        }

        Ok(())
    }

    fn check_while_statement(&mut self, while_statement: &WhileStatement) -> Result<(), Error> {
        let test_type = self.check_expression(&while_statement.test)?;
        if test_type != Some(Type::Bool) {
            return Err(error(
                while_statement.test.region.clone(),
                format!("while loops can only check booleans, found {:?}", test_type),
            ));
        };

        self.check_block(&while_statement.block)?;

        Ok(())
    }

    fn check_each(&mut self, each: &EachStatement) -> Result<(), Error> {
        let right_type = self.check_expression(&each.right)?.ok_or(error(
            each.right.region.clone(),
            "tried to loop over void type".to_string(),
        ))?;
        if let Type::List(left_type) = right_type {
            if let Some(left_type) = left_type {
                self.enter_block();

                self.declare_identifier(Identifier {
                    identifier: each.left.clone(),
                    type_: *left_type,
                });
                self.check_block(&each.block)?;

                self.exit_block();
            } else {
                return Err(error(
                    each.right.region.clone(),
                    "failed to find inner type of list".to_string(),
                ));
            }
        } else {
            return Err(error(
                each.right.region.clone(),
                format!("expected to loop over list, found {:?}", right_type),
            ));
        }

        Ok(())
    }

    fn check_return(
        &mut self,
        return_statement: &ReturnStatement,
        region: &Region,
    ) -> Result<(), Error> {
        let function_type = self.get_current_return_type().ok_or(error(
            region.clone(),
            "return statement outside of function body".to_string(),
        ))?;

        let _return_type = match &return_statement.expression {
            Some(expression) => self.check_expression(expression)?,
            None => {
                if function_type.is_none() {
                    return Ok(());
                } else {
                    return Err(error(region.clone(), "missing return value".to_string()));
                }
            }
        };

        todo!()
        // match &return_type {
        //     Some(t) => {
        //         if Some(t.clone()) == *function_type {
        //             Ok(())
        //         } else {
        //             Err(error(
        //                 region.clone(),
        //                 format!(
        //                     "return statement type mismatch, (function body: {:?}, return statement: {:?})",
        //                     function_type, return_type
        //                 ),
        //             ))
        //         }
        //     }
        //     None => Ok(()),
        // }
    }

    fn check_statement(&mut self, statement: &Statement) -> Result<(), Error> {
        match &statement.value {
            StatementValue::Block(block) => self.check_block(block),
            StatementValue::VariableDeclaration(var) => {
                self.check_variable_declaration(var, &statement.region)
            }
            StatementValue::FunctionDeclaration(function) => {
                self.check_function_declaration(function)
            }
            StatementValue::If(if_statement) => self.check_if_statement(if_statement),
            StatementValue::While(while_statement) => self.check_while_statement(while_statement),
            StatementValue::Each(each) => self.check_each(each),
            StatementValue::Expression(expression) => self.check_expression(expression).map(|_| ()),
            StatementValue::Return(return_statement) => {
                self.check_return(return_statement, &statement.region)
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
        Checker {
            scopes: vec![],
            function_bodies: vec![],
        }
    }
}

pub fn check(program: &Vec<Statement>) -> Result<(), Error> {
    let mut checker = Checker::new();
    checker.check(program)?;
    dbg!(checker);
    Ok(())
}
