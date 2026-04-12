use crate::{
    ast::*,
    checker::{Type, check},
    error::Error,
    parser::parse,
};
use std::rc::Rc;
use variantly::Variantly;

mod builtins;

#[derive(Debug, PartialEq, Clone)]
pub struct Argument {
    pub identifier: String,
    pub type_: Type,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(unpredictable_function_pointer_comparisons)]
#[allow(unused)]
pub enum Function {
    Custom {
        args: Vec<Argument>,
        body: Vec<Statement>,
    },
    Builtin(fn(args: Vec<Value>) -> Value),
}

#[derive(Variantly, Debug, PartialEq, Clone)]
#[allow(unused)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    List(Vec<Value>),
    Reference(Rc<Value>),
    Void,
    Function(Function),
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
#[allow(unused)]
enum Scope {
    Block,
    Identifier(Identifier),
}

struct Evaluator {
    scopes: Vec<Scope>,
}

impl<'a> Evaluator {
    fn _exit_block(&mut self) {
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
    fn _enter_block(&mut self) {
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
        Self {
            scopes: vec![Scope::Identifier(Identifier {
                identifier: "puts".to_string(),
                value: Value::Function(Function::Builtin(builtins::puts)),
            })],
        }
    }

    fn eval_call(&mut self, call: &FunctionCallExpression) -> Value {
        let callee = self.eval_expression(&call.callee);

        match callee.unwrap_function() {
            Function::Builtin(f) => {
                let arg_values = call
                    .arguments
                    .iter()
                    .map(|a| self.eval_expression(a))
                    .collect();

                f(arg_values)
            }
            Function::Custom { args: _, body: _ } => {
                todo!()
            }
        }
    }

    fn eval_expression(&mut self, expression: &Expression) -> Value {
        match &expression.value {
            ExpressionValue::Bool(b) => Value::Bool(*b),
            ExpressionValue::FunctionCall(call) => self.eval_call(call),
            ExpressionValue::Identifier(id) => self.get_identifier(&id.identifier).clone(),
            ExpressionValue::String(s) => Value::String(s.clone()),
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
            StatementValue::Expression(expr) => {
                self.eval_expression(expr);
            }
            _ => {
                todo!()
            }
        }
    }

    pub fn eval_ast(&mut self, ast: &[Statement]) -> Value {
        for statement in ast {
            self.eval_statement(statement);
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
