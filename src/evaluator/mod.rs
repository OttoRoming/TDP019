use crate::{ast::*, checker::check, error::Error, parser::parse};
use std::{cell::RefCell, process::exit, rc::Rc};

pub mod builtins;
pub mod values;

#[cfg(test)]
mod test;

use values::{ControlFlow, Exception, Function, Return, Value};

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
    match &*value.borrow() {
        Value::List(l) => Rc::new(RefCell::new(Value::List(
            l.iter().map(|v| deep_copy(Rc::clone(v))).collect(),
        ))),
        _ => Rc::new(RefCell::new(value.borrow().clone())),
    }
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
                        scope,
                        parameters,
                        body: Rc::clone(&body),
                    }))),
                );
            }

            let block_result = eval_block(fun_scope, &body);

            if let Some(ControlFlow::Return(r)) = block_result {
                r.value
                    .unwrap_or_else(|| Rc::new(RefCell::new(Value::Void)))
            } else {
                Rc::new(RefCell::new(Value::Void))
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
                Value::Int(l) => *l = r.unwrap_int(),
                Value::Float(l) => *l = r.unwrap_float(),
                Value::Bool(l) => *l = r.unwrap_bool(),
                Value::String(l) => {
                    l.clear();
                    l.push_str(r.unwrap_str())
                }
                Value::List(l) => {
                    l.clear();
                    l.extend(r.unwrap_list_ref().iter().map(Rc::clone));
                }
                Value::Reference(l) => *l = r.unwrap_reference(),
                Value::Function(l) => *l = r.clone().unwrap_function(),
                Value::Void => unreachable!(),
            },
            AssignmentOperator::Add => match &mut *l {
                Value::Int(l) => *l += r.unwrap_int(),
                Value::Float(l) => *l += r.unwrap_float(),
                Value::String(l) => l.push_str(r.unwrap_str()),
                _ => unreachable!(),
            },
            AssignmentOperator::Divide => match &mut *l {
                Value::Int(l) => *l /= r.unwrap_int(),
                Value::Float(l) => *l /= r.unwrap_float(),
                _ => unreachable!(),
            },
            AssignmentOperator::Modulo => match &mut *l {
                Value::Int(l) => *l %= r.unwrap_int(),
                Value::Float(l) => *l %= r.unwrap_float(),
                _ => unreachable!(),
            },
            AssignmentOperator::Multiply => match &mut *l {
                Value::Int(l) => *l *= r.unwrap_int(),
                Value::Float(l) => *l *= r.unwrap_float(),
                _ => unreachable!(),
            },
            AssignmentOperator::Subtract => match &mut *l {
                Value::Int(l) => *l -= r.unwrap_int(),
                Value::Float(l) => *l -= r.unwrap_float(),
                _ => unreachable!(),
            },
            AssignmentOperator::And => match &mut *l {
                Value::Bool(l) => *l &= r.unwrap_bool(),
                _ => unreachable!(),
            },
            AssignmentOperator::Or => match &mut *l {
                Value::Bool(l) => *l |= r.unwrap_bool(),
                _ => unreachable!(),
            },
            AssignmentOperator::Append => match &mut *l {
                Value::List(l) => {
                    l.push(Rc::new(RefCell::new(r.clone())));
                }
                _ => unreachable!(),
            },
        }
    }

    left
}

fn eval_update(scope: Rc<Scope>, update: &UpdateExpression) -> Rc<RefCell<Value>> {
    let updatee_value = eval_expression(scope, &update.updatee);

    {
        let mut u = updatee_value.borrow_mut();

        match update.operator {
            UpdateOperator::Increment => match &mut *u {
                Value::Int(u) => *u += 1,
                Value::Float(u) => *u += 1.0,
                _ => unreachable!(),
            },
            UpdateOperator::Decrement => match &mut *u {
                Value::Int(u) => *u -= 1,
                Value::Float(u) => *u -= 1.0,
                _ => unreachable!(),
            },
        }
    }

    updatee_value
}

fn eval_binary(scope: Rc<Scope>, binary: &BinaryExpression) -> Rc<RefCell<Value>> {
    let left = eval_expression(Rc::clone(&scope), &binary.left);
    let right = eval_expression(scope, &binary.right);

    let l = left.borrow();
    let r = right.borrow();

    Rc::new(RefCell::new(match &binary.operator {
        BinaryOperator::Add => match &*l {
            Value::Int(l) => Value::Int(l + r.unwrap_int()),
            Value::Float(l) => Value::Float(l + r.unwrap_float()),
            Value::String(l) => Value::String(format!("{}{}", l, r.unwrap_str())),
            _ => unreachable!(),
        },
        BinaryOperator::Subtract => match *l {
            Value::Int(l) => Value::Int(l - r.unwrap_int()),
            Value::Float(l) => Value::Float(l - r.unwrap_float()),
            _ => unreachable!(),
        },
        BinaryOperator::Multiply => match *l {
            Value::Int(l) => Value::Int(l * r.unwrap_int()),
            Value::Float(l) => Value::Float(l * r.unwrap_float()),
            _ => unreachable!(),
        },
        BinaryOperator::Divide => match *l {
            Value::Int(l) => Value::Int(l / r.unwrap_int()),
            Value::Float(l) => Value::Float(l / r.unwrap_float()),
            _ => unreachable!(),
        },
        BinaryOperator::Modulo => match *l {
            Value::Int(l) => Value::Int(l % r.unwrap_int()),
            Value::Float(l) => Value::Float(l % r.unwrap_float()),
            _ => unreachable!(),
        },
        BinaryOperator::Equals => Value::Bool(match &*l {
            Value::Int(l) => *l == r.unwrap_int(),
            Value::Float(l) => *l == r.unwrap_float(),
            Value::Bool(l) => *l == r.unwrap_bool(),
            Value::Reference(l) => *l == r.unwrap_reference(),
            Value::String(l) => l == r.unwrap_str(),
            Value::List(l) => l == r.unwrap_list_ref(),
            _ => unreachable!(),
        }),
        BinaryOperator::NotEquals => Value::Bool(match &*l {
            Value::Int(l) => *l != r.unwrap_int(),
            Value::Float(l) => *l != r.unwrap_float(),
            Value::Bool(l) => *l != r.unwrap_bool(),
            Value::Reference(l) => *l != r.unwrap_reference(),
            Value::String(l) => l != r.unwrap_str(),
            Value::List(l) => l != r.unwrap_list_ref(),
            _ => unreachable!(),
        }),
        BinaryOperator::GreaterThan => Value::Bool(match *l {
            Value::Int(l) => l > r.unwrap_int(),
            Value::Float(l) => l > r.unwrap_float(),
            _ => unreachable!(),
        }),
        BinaryOperator::GreaterThanOrEqual => Value::Bool(match *l {
            Value::Int(l) => l >= r.unwrap_int(),
            Value::Float(l) => l >= r.unwrap_float(),
            _ => unreachable!(),
        }),
        BinaryOperator::LessThan => Value::Bool(match *l {
            Value::Int(l) => l < r.unwrap_int(),
            Value::Float(l) => l < r.unwrap_float(),
            _ => unreachable!(),
        }),
        BinaryOperator::LessThanOrEqual => Value::Bool(match *l {
            Value::Int(l) => l <= r.unwrap_int(),
            Value::Float(l) => l <= r.unwrap_float(),
            _ => unreachable!(),
        }),
        BinaryOperator::And => Value::Bool(match *l {
            Value::Bool(l) => l && r.unwrap_bool(),
            _ => unreachable!(),
        }),
        BinaryOperator::Or => Value::Bool(match *l {
            Value::Bool(l) => l || r.unwrap_bool(),
            _ => unreachable!(),
        }),
    }))
}

fn eval_unary(scope: Rc<Scope>, unary: &UnaryExpression) -> Rc<RefCell<Value>> {
    let right = eval_expression(Rc::clone(&scope), &unary.right);

    if &unary.operator == &UnaryOperator::Dereference {
        return right.borrow().unwrap_reference();
    }

    Rc::new(RefCell::new(match &unary.operator {
        UnaryOperator::Negate => match *right.borrow() {
            Value::Int(r) => Value::Int(-r),
            Value::Float(r) => Value::Float(-r),
            _ => unreachable!(),
        },
        UnaryOperator::Not => Value::Bool(match *right.borrow() {
            Value::Bool(r) => !r,
            _ => unreachable!(),
        }),
        UnaryOperator::Dereference => match &*right.borrow() {
            Value::Reference(r) => r.borrow().clone(),
            _ => unreachable!(),
        },
        UnaryOperator::Reference => Value::Reference(right),
    }))
}

fn eval_index(scope: Rc<Scope>, index: &IndexExpression) -> Rc<RefCell<Value>> {
    let collection = eval_expression(Rc::clone(&scope), &index.collection);
    let index_value = eval_expression(Rc::clone(&scope), &index.index);

    let collection_borrow = collection.borrow();

    let list = collection_borrow.unwrap_list_ref();

    Rc::clone(
        list.get(index_value.borrow().unwrap_int() as usize)
            .unwrap(),
    )
}

fn eval_list(scope: Rc<Scope>, list: &[Expression]) -> Rc<RefCell<Value>> {
    let values: Vec<Rc<RefCell<Value>>> = list
        .iter()
        .map(|e| eval_expression(Rc::clone(&scope), e))
        .map(deep_copy)
        .collect();

    Rc::new(RefCell::new(Value::List(values)))
}

fn eval_expression(scope: Rc<Scope>, expression: &Expression) -> Rc<RefCell<Value>> {
    match &expression.value {
        ExpressionValue::FunctionCall(call) => eval_call(scope, call),
        ExpressionValue::Identifier(id) => eval_identifier(scope, id),
        ExpressionValue::Assign(assign) => eval_assign(scope, assign),
        ExpressionValue::Update(update) => eval_update(scope, update),
        ExpressionValue::Binary(binary) => eval_binary(scope, binary),
        ExpressionValue::Unary(unary) => eval_unary(scope, unary),
        ExpressionValue::Index(index) => eval_index(scope, index),
        ExpressionValue::List(list) => eval_list(scope, list),
        ExpressionValue::Bool(b) => Rc::new(RefCell::new(Value::Bool(*b))),
        ExpressionValue::String(s) => Rc::new(RefCell::new(Value::String(s.clone()))),
        ExpressionValue::Int(i) => Rc::new(RefCell::new(Value::Int(*i))),
        ExpressionValue::Float(f) => Rc::new(RefCell::new(Value::Float(*f))),
    }
}

#[must_use]
fn eval_variable_declaration(scope: Rc<Scope>, var: &VariableDeclarationStatement) -> Rc<Scope> {
    let value = eval_expression(Rc::clone(&scope), &var.expression);

    Scope::new_id(scope, var.identifier.clone(), deep_copy(value))
}

#[must_use]
fn eval_if_branch(scope: Rc<Scope>, branch: &IfBranch) -> Option<ControlFlow> {
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
fn eval_if_statement(scope: Rc<Scope>, if_statement: &IfStatement) -> Option<ControlFlow> {
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
    let value = return_statement
        .expression
        .as_ref()
        .map(|expression| eval_expression(scope, expression));

    Return { value }
}

#[must_use]
fn eval_block(scope: Rc<Scope>, block: &Block) -> Option<ControlFlow> {
    let mut block_scope = Scope::new_block(Some(scope));
    let mut control_flow: Option<ControlFlow> = None;

    for statement in block.statements.iter() {
        (block_scope, control_flow) = eval_statement(block_scope, statement);
        if control_flow.is_some() {
            break;
        }
    }

    control_flow
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
fn eval_while(scope: Rc<Scope>, while_statement: &WhileStatement) -> Option<ControlFlow> {
    let mut control_flow: Option<ControlFlow> = None;

    loop {
        let test = eval_expression(Rc::clone(&scope), &while_statement.test);
        if !test.borrow().unwrap_bool() {
            break;
        }

        control_flow = eval_block(Rc::clone(&scope), &while_statement.block);
        if let Some(cf) = &control_flow {
            match cf {
                ControlFlow::Continue => {
                    control_flow = None;
                    continue;
                }
                ControlFlow::Break => {
                    control_flow = None;
                    break;
                }
                _ => break,
            }
        }
    }

    control_flow
}

fn eval_each(scope: Rc<Scope>, each: &EachStatement) -> Option<ControlFlow> {
    let mut control_flow: Option<ControlFlow> = None;

    let right = eval_expression(Rc::clone(&scope), &each.right);
    let right_borrow = right.borrow();
    let right_list = right_borrow.unwrap_list_ref();

    for i in right_list {
        let each_scope = Scope::new_id(
            Rc::clone(&scope),
            each.left.clone(),
            deep_copy(Rc::clone(i)),
        );

        control_flow = eval_block(each_scope, &each.block);
        if let Some(cf) = &control_flow {
            match cf {
                ControlFlow::Continue => {
                    control_flow = None;
                    continue;
                }
                ControlFlow::Break => {
                    control_flow = None;
                    break;
                }
                _ => break,
            }
        }
    }

    control_flow
}

#[must_use]
fn eval_throw(scope: Rc<Scope>, throw: &Throw) -> Exception {
    let message = eval_expression(scope, &throw.message)
        .borrow()
        .clone()
        .unwrap_string();

    Exception { message }
}

#[must_use]
fn eval_try_catch(scope: Rc<Scope>, try_catch: &TryCatch) -> Option<ControlFlow> {
    let try_result = eval_block(Rc::clone(&scope), &try_catch.try_block);

    if let Some(ControlFlow::Exception(e)) = try_result {
        let mut catch_scope = scope;
        if let Some(exception_identifier) = &try_catch.exception_identifier {
            catch_scope = Scope::new_id(
                catch_scope,
                exception_identifier.clone(),
                Rc::new(RefCell::new(Value::String(e.message))),
            );
        }

        eval_block(catch_scope, &try_catch.catch_block)
    } else {
        try_result
    }
}

#[must_use]
fn eval_statement(scope: Rc<Scope>, statement: &Statement) -> (Rc<Scope>, Option<ControlFlow>) {
    let mut new_scope = Rc::clone(&scope);
    let mut control_flow: Option<ControlFlow> = None;

    match &statement.value {
        StatementValue::VariableDeclaration(var) => {
            new_scope = eval_variable_declaration(scope, var);
        }
        StatementValue::If(if_statement) => control_flow = eval_if_statement(scope, if_statement),
        StatementValue::Return(return_statement) => {
            control_flow = Some(ControlFlow::Return(eval_return(scope, return_statement)))
        }
        StatementValue::Block(block) => control_flow = eval_block(scope, block),
        StatementValue::Expression(expr) => {
            eval_expression(scope, expr);
        }
        StatementValue::FunctionDeclaration(fun) => {
            new_scope = eval_function_declaration(scope, fun);
        }
        StatementValue::While(while_statement) => control_flow = eval_while(scope, while_statement), // _ => {
        StatementValue::Each(each) => control_flow = eval_each(scope, each),
        StatementValue::Throw(throw) => {
            control_flow = Some(ControlFlow::Exception(eval_throw(scope, throw)))
        }
        StatementValue::TryCatch(try_catch) => control_flow = eval_try_catch(scope, try_catch),
        StatementValue::Continue => control_flow = Some(ControlFlow::Continue),
        StatementValue::Break => control_flow = Some(ControlFlow::Break), // _ => todo!(),
    };

    (new_scope, control_flow)
}

pub fn eval_ast(ast: &[Statement]) -> Option<Rc<RefCell<Value>>> {
    // let mut scope = Scope::new_block(None);
    // scope = Scope::new_builtin(scope, "puts", builtins::puts);
    let mut scope = builtins::evaluator_scopes();

    let mut control_flow: Option<ControlFlow> = None;

    for statement in ast {
        if control_flow.is_some() {
            break;
        }

        (scope, control_flow) = eval_statement(Rc::clone(&scope), statement);
    }

    match control_flow {
        Some(control_flow) => match control_flow {
            ControlFlow::Break => {
                println!("broke outside of loop");
                exit(2);
            }
            ControlFlow::Continue => {
                println!("continued outside of loop");
                exit(3);
            }
            ControlFlow::Exception(e) => {
                println!("unhandled runtime exception, {}", e.message);
                exit(1);
            }
            ControlFlow::Return(r) => r.value,
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
