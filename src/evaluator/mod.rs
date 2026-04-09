use crate::{
    ast::*,
    ast::{Statement, VariableDeclarationStatement},
    checker::check,
    error::Error,
    parser::parse,
};
use std::rc::Rc;
use variantly::Variantly;

#[derive(Variantly, Debug, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    List(Vec<Value>),
    Reference(Rc<Value>),
    Void,
    Function {},
}

#[allow(dead_code)]
enum ControlFlow {
    Return(Value),
}

#[derive(Debug, PartialEq)]
struct Identifier {
    pub identifier: String,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
enum Scope {
    Block,
    Identifier(Identifier),
}

struct Evaluator {
    scopes: Vec<Scope>,
}

impl<'a> Evaluator {
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
    fn get_identifier(&'a mut self, id: &str) -> &'a Value {
        for scope in self.scopes.iter().rev() {
            if let Scope::Identifier(scope_id) = scope
                && scope_id.identifier == id
            {
                return &scope_id.value;
            }
        }

        panic!("tried to get undeclared identifier")
    }

    pub fn new() -> Self {
        Self { scopes: vec![] }
    }

    fn eval_call(&mut self, call: &FunctionCallExpression) -> Value {
        todo!()
    }

    fn eval_expression(&mut self, expression: &Expression) -> Value {
        match &expression.value {
            ExpressionValue::Bool(b) => Value::Bool(*b),
            ExpressionValue::FunctionCall(call) => self.eval_call(call),
            _ => todo!(),
        }
    }

    fn eval_variable_declaration(&mut self, var: &VariableDeclarationStatement) {
        let value = self.eval_expression(&var.expression);

        self.declare_identifier(Identifier {
            identifier: var.identifier.clone(),
            value,
        });
    }

    fn eval_statement(&mut self, statement: &Statement) {
        match &statement.value {
            StatementValue::VariableDeclaration(var) => self.eval_variable_declaration(var),
            _ => {
                todo!()
            }
        }
    }

    pub fn eval_ast(&mut self, ast: &[Statement]) -> Value {
        for statement in ast {
            self.eval_statement(&statement);
        }

        Value::Bool(true)
    }
}

pub fn eval(source: &str) -> Result<Value, Error> {
    let ast = parse(source)?;
    check(&ast)?;

    let mut evaluator = Evaluator::new();
    Ok(evaluator.eval_ast(&ast))
}
