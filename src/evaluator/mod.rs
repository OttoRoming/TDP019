use crate::{
    ast::*,
    checker::{Type, check},
    error::Error,
    parser::parse,
};
use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};
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
    Builtin(fn(args: Vec<Rc<RefCell<Value>>>) -> Rc<RefCell<Value>>),
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
    pub value: Rc<RefCell<Value>>,
}

struct Scope {
    upper: Option<Rc<Scope>>,
    data: ScopeData,
}

impl Scope {
    pub fn new_block(upper: Option<Rc<Scope>>) -> Rc<Self> {
        Rc::new(Self {
            upper,
            data: ScopeData::Block,
        })
    }
    pub fn new_id(upper: Rc<Scope>, id: String, value: Rc<RefCell<Value>>) -> Rc<Self> {
        Rc::new(Self {
            upper: Some(upper),
            data: ScopeData::Identifier(Identifier {
                identifier: id,
                value,
            }),
        })
    }
}

#[derive(Debug, PartialEq)]
enum ScopeData {
    Block,
    Identifier(Identifier),
}

fn eval_call(scope: Rc<Scope>, call: &FunctionCallExpression) -> Rc<RefCell<Value>> {
    let callee = eval_expression(Rc::clone(&scope), &call.callee);

    match callee.borrow().clone().unwrap_function() {
        Function::Builtin(f) => {
            let arg_values: Vec<Rc<RefCell<Value>>> = call
                .arguments
                .iter()
                .map(|a| eval_expression(Rc::clone(&scope), a))
                .collect();

            f(arg_values)
        }
        Function::Custom { args: _, body: _ } => {
            todo!()
        }
    }
}

fn eval_identifier(scope: Rc<Scope>, identifier: &IdentifierExpression) -> Rc<RefCell<Value>> {
    let upper = Rc::clone(
        scope
            .upper
            .as_ref()
            .expect("reached top scope in eval_identifier"),
    );

    match &scope.data {
        ScopeData::Block => eval_identifier(upper, identifier),
        ScopeData::Identifier(id) => {
            if identifier.identifier == id.identifier {
                return id.value.clone();
            } else {
                eval_identifier(upper, identifier)
            }
        }
    }
}

fn eval_assign(scope: Rc<Scope>, assign: &AssignmentExpression) -> Rc<RefCell<Value>> {
    let left = eval_expression(Rc::clone(&scope), &assign.assignee);
    let right = eval_expression(scope, &assign.right);

    {
        match assign.operator {
            AssignmentOperator::Equals => {
                let mut v = left.borrow_mut();
                match &mut *v {
                    Value::String(s) => {
                        s.clear();
                        s.push_str(right.borrow().string_ref().unwrap())
                    }
                    Value::Bool(b) => *b = *right.borrow().bool_ref().unwrap(),
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }

    left
}

fn eval_expression(scope: Rc<Scope>, expression: &Expression) -> Rc<RefCell<Value>> {
    match &expression.value {
        ExpressionValue::FunctionCall(call) => eval_call(scope, call),
        ExpressionValue::Identifier(id) => eval_identifier(scope, id),
        ExpressionValue::Assign(assign) => eval_assign(scope, assign),
        ExpressionValue::Bool(b) => Rc::new(RefCell::new(Value::Bool(*b))),
        ExpressionValue::String(s) => Rc::new(RefCell::new(Value::String(s.clone()))),
        ExpressionValue::Int(i) => Rc::new(RefCell::new(Value::Int(*i))),
        ExpressionValue::Float(f) => Rc::new(RefCell::new(Value::Float(*f))),
        _ => todo!(),
    }
}

fn eval_variable_declaration(scope: Rc<Scope>, var: &VariableDeclarationStatement) -> Rc<Scope> {
    let value = eval_expression(Rc::clone(&scope), &var.expression);
    let new_scope = Scope::new_id(scope, var.identifier.clone(), value);
    return new_scope;
    // self.declare_identifier(Identifier {
    //     identifier: var.identifier.clone(),
    //     value,
    // });
}

fn eval_statement(scope: Rc<Scope>, statement: &Statement) -> Rc<Scope> {
    match &statement.value {
        StatementValue::VariableDeclaration(var) => eval_variable_declaration(scope, var),
        StatementValue::Expression(expr) => {
            eval_expression(Rc::clone(&scope), expr);
            scope
        }
        _ => {
            todo!()
        }
    }
}

pub fn eval_ast(ast: &[Statement]) -> Value {
    let mut scope = Scope::new_block(None);
    scope = Scope::new_id(
        scope,
        "puts".to_string(),
        Rc::new(RefCell::new(Value::Function(Function::Builtin(
            builtins::puts,
        )))),
    );

    for statement in ast {
        scope = eval_statement(Rc::clone(&scope), statement);
    }

    Value::Bool(true)
}
// }

pub fn eval(source: &str) -> Result<Value, Error> {
    let ast = parse(source)?;
    check(&ast)?;

    // let mut evaluator = Evaluator::new();
    Ok(eval_ast(&ast))
}
