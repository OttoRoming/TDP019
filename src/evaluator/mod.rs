use crate::{ast::*, checker::check, error::Error, parser::parse};
use std::{cell::RefCell, rc::Rc};

mod builtins;
pub mod values;

#[cfg(test)]
mod test;

use values::{Function, Return, Value};

#[derive(Debug, PartialEq)]
struct Identifier {
    pub identifier: String,
    pub value: Rc<RefCell<Value>>,
}

#[derive(Debug, PartialEq)]
pub struct Scope {
    upper: Option<Rc<Scope>>,
    data: ScopeData,
}

fn deep_copy(value: Rc<RefCell<Value>>) -> Rc<RefCell<Value>> {
    Rc::new(RefCell::new(value.borrow().clone()))
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
    pub fn new_builtin(upper: Rc<Scope>, id: &str, builtin: builtins::Function) -> Rc<Self> {
        Rc::new(Self {
            upper: Some(upper),
            data: ScopeData::Identifier(Identifier {
                identifier: id.to_string(),
                value: Rc::new(RefCell::new(Value::Function(Function::Builtin(builtin)))),
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
    let callee_id = if let ExpressionValue::Identifier(id) = &call.callee.value {
        Some(&id.identifier)
    } else {
        None
    };

    let arg_values: Vec<Rc<RefCell<Value>>> = call
        .arguments
        .iter()
        .map(|a| eval_expression(Rc::clone(&scope), a))
        .collect();

    match callee.borrow().clone().unwrap_function() {
        Function::Builtin(f) => f(arg_values),
        Function::Custom {
            scope,
            parameters,
            body,
        } => {
            let mut fun_scope = Scope::new_block(Some(Rc::clone(&scope)));

            for (i, value) in arg_values.iter().enumerate() {
                fun_scope = Scope::new_id(
                    fun_scope,
                    parameters[i].identifier.clone(),
                    deep_copy(Rc::clone(value)),
                );
            }

            // allow for recursive function calling by declaring
            // the current function in it's scope
            if let Some(id) = callee_id {
                fun_scope = Scope::new_id(
                    fun_scope,
                    id.to_string(),
                    Rc::new(RefCell::new(Value::Function(Function::Custom {
                        scope: scope,
                        parameters: parameters,
                        body: Rc::clone(&body),
                    }))),
                );
            }

            let block_result = eval_block(fun_scope, &body);

            match block_result {
                Some(r) => match r.value {
                    Some(v) => v,
                    None => Rc::new(RefCell::new(Value::Void)),
                },
                None => Rc::new(RefCell::new(Value::Void)),
            }
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
                Rc::clone(&id.value)
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
        let mut l = left.borrow_mut();
        let r = right.borrow();

        match assign.operator {
            AssignmentOperator::Equals => match &mut *l {
                Value::Int(i) => *i = r.unwrap_int(),
                Value::Float(f) => *f = r.unwrap_float(),
                Value::Bool(b) => *b = r.unwrap_bool(),
                Value::String(s) => {
                    s.clear();
                    s.push_str(r.unwrap_str())
                }
                Value::List(l) => {
                    l.clear();
                    l.extend(r.clone().unwrap_list());
                }
                Value::Reference(reference) => *reference = r.unwrap_reference(),
                Value::Function(f) => *f = r.clone().unwrap_function(),
                Value::Void => unreachable!(),
            },
            AssignmentOperator::Add => match &mut *l {
                Value::Int(i) => *i += r.unwrap_int(),
                Value::Float(f) => *f += r.unwrap_float(),
                _ => unreachable!(),
            },
            AssignmentOperator::Divide => match &mut *l {
                Value::Int(i) => *i /= r.unwrap_int(),
                Value::Float(f) => *f /= r.unwrap_float(),
                _ => unreachable!(),
            },
            AssignmentOperator::Modulo => match &mut *l {
                Value::Int(i) => *i %= r.unwrap_int(),
                Value::Float(f) => *f %= r.unwrap_float(),
                _ => unreachable!(),
            },
            AssignmentOperator::Multiply => match &mut *l {
                Value::Int(i) => *i *= r.unwrap_int(),
                Value::Float(f) => *f *= r.unwrap_float(),
                _ => unreachable!(),
            },
            AssignmentOperator::Subtract => match &mut *l {
                Value::Int(i) => *i -= r.unwrap_int(),
                Value::Float(f) => *f -= r.unwrap_float(),
                _ => unreachable!(),
            },
            AssignmentOperator::And => match &mut *l {
                Value::Bool(b) => *b &= r.unwrap_bool(),
                _ => unreachable!(),
            },
            AssignmentOperator::Or => match &mut *l {
                Value::Bool(b) => *b |= r.unwrap_bool(),
                _ => unreachable!(),
            },
        }
    }

    left
}

fn eval_binary(scope: Rc<Scope>, binary: &BinaryExpression) -> Rc<RefCell<Value>> {
    let left = eval_expression(Rc::clone(&scope), &binary.left);
    let right = eval_expression(scope, &binary.right);

    let l = left.borrow();
    let r = right.borrow();

    Rc::new(RefCell::new(match &binary.operator {
        BinaryOperator::Equals => Value::Bool(match *l {
            Value::Int(i) => i == r.unwrap_int(),
            _ => todo!(),
        }),
        BinaryOperator::LessThan => Value::Bool(match *l {
            Value::Int(i) => i < r.unwrap_int(),
            _ => todo!(),
        }),
        BinaryOperator::Add => match *l {
            Value::Int(i) => Value::Int(i + r.unwrap_int()),
            _ => todo!(),
        },
        BinaryOperator::Subtract => match *l {
            Value::Int(i) => Value::Int(i - r.unwrap_int()),
            _ => todo!(),
        },
        _ => todo!(),
    }))
}

fn eval_expression(scope: Rc<Scope>, expression: &Expression) -> Rc<RefCell<Value>> {
    match &expression.value {
        ExpressionValue::FunctionCall(call) => eval_call(scope, call),
        ExpressionValue::Identifier(id) => eval_identifier(scope, id),
        ExpressionValue::Assign(assign) => eval_assign(scope, assign),
        ExpressionValue::Binary(binary) => eval_binary(scope, binary),
        ExpressionValue::Bool(b) => Rc::new(RefCell::new(Value::Bool(*b))),
        ExpressionValue::String(s) => Rc::new(RefCell::new(Value::String(s.clone()))),
        ExpressionValue::Int(i) => Rc::new(RefCell::new(Value::Int(*i))),
        ExpressionValue::Float(f) => Rc::new(RefCell::new(Value::Float(*f))),
        _ => todo!(),
    }
}

#[must_use]
fn eval_variable_declaration(scope: Rc<Scope>, var: &VariableDeclarationStatement) -> Rc<Scope> {
    let value = eval_expression(Rc::clone(&scope), &var.expression);

    Scope::new_id(scope, var.identifier.clone(), deep_copy(value))
}

#[must_use]
fn eval_if_branch(scope: Rc<Scope>, branch: &IfBranch) -> Option<Return> {
    match branch {
        IfBranch::Elif(elif) => {
            let test = eval_expression(Rc::clone(&scope), &elif.test);

            if test.borrow().unwrap_bool() {
                eval_block(scope, &elif.block)
            } else if let Some(inner_branch) = &*elif.branch {
                eval_if_branch(scope, inner_branch)
            } else {
                None
            }
        }
        IfBranch::Else(els) => eval_block(scope, &els.block),
    }
}

#[must_use]
fn eval_if_statement(scope: Rc<Scope>, if_statement: &IfStatement) -> Option<Return> {
    let test = eval_expression(Rc::clone(&scope), &if_statement.test);

    if test.borrow().unwrap_bool() {
        eval_block(scope, &if_statement.block)
    } else if let Some(branch) = &if_statement.branch {
        eval_if_branch(scope, branch)
    } else {
        None
    }
}

#[must_use]
fn eval_return(scope: Rc<Scope>, return_statement: &ReturnStatement) -> Return {
    let value = match &return_statement.expression {
        Some(expression) => Some(eval_expression(scope, expression)),
        None => None,
    };

    Return { value }
}

#[must_use]
fn eval_block(scope: Rc<Scope>, block: &Block) -> Option<Return> {
    let mut block_scope = Scope::new_block(Some(scope));
    let mut return_value: Option<Return> = None;

    for statement in block.statements.iter() {
        (block_scope, return_value) = eval_statement(block_scope, statement);
        if return_value.is_some() {
            break;
        }
    }

    return_value
}

#[must_use]
fn eval_function_declaration(scope: Rc<Scope>, fun: &FunctionDeclarationStatement) -> Rc<Scope> {
    Scope::new_id(
        Rc::clone(&scope),
        fun.identifier.clone(),
        Rc::new(RefCell::new(Value::Function(Function::Custom {
            scope,
            parameters: fun.parameters.clone(),
            body: Rc::new(fun.block.clone()),
        }))),
    )
}

#[must_use]
fn eval_statement(scope: Rc<Scope>, statement: &Statement) -> (Rc<Scope>, Option<Return>) {
    let mut new_scope = Rc::clone(&scope);
    let mut return_value: Option<Return> = None;

    match &statement.value {
        StatementValue::VariableDeclaration(var) => {
            new_scope = eval_variable_declaration(scope, var);
        }
        StatementValue::If(if_statement) => return_value = eval_if_statement(scope, if_statement),
        StatementValue::Return(return_statement) => {
            return_value = Some(eval_return(scope, return_statement))
        }
        StatementValue::Block(block) => return_value = eval_block(scope, block),
        StatementValue::Expression(expr) => {
            eval_expression(scope, expr);
        }
        StatementValue::FunctionDeclaration(fun) => {
            new_scope = eval_function_declaration(scope, fun);
        }
        _ => {
            todo!()
        }
    };

    (new_scope, return_value)
}

pub fn eval_ast(ast: &[Statement]) -> Option<Rc<RefCell<Value>>> {
    let mut scope = Scope::new_block(None);
    scope = Scope::new_builtin(scope, "puts", builtins::puts);

    let mut return_value: Option<Return> = None;

    for statement in ast {
        if return_value.is_some() {
            break;
        }

        (scope, return_value) = eval_statement(Rc::clone(&scope), statement);
    }

    match return_value {
        Some(r) => match r.value {
            Some(v) => Some(v),
            None => None,
        },
        None => None,
    }
}

pub fn eval(source: &str) -> Result<Value, Error> {
    let ast = parse(source)?;
    check(&ast)?;

    Ok(match eval_ast(&ast) {
        Some(v) => v.borrow().clone(),
        None => Value::Int(0),
    })
}
