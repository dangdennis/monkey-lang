// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::rc::Rc;

// use crate::ast::*;
// use crate::object::{
//     Array, Boolean, Builtin, Environment, Error, Function, HashObj, HashPair, Integer, Null,
//     Object, ObjectType, ReturnValue, StringObj,
// };

// lazy_static::lazy_static! {
//     static ref NULL: Object = Object::Null(Null::new());
//     static ref TRUE: Object = Object::Boolean(Boolean::new(true));
//     static ref FALSE: Object = Object::Boolean(Boolean::new(false));
// }

// pub fn eval(node: &Program, env: Rc<RefCell<Environment>>) -> Object {
//     eval_program(&node.statements, env)
// }

// pub fn eval_statement(stmt: &Statement, env: Rc<RefCell<Environment>>) -> Object {
//     match stmt {
//         Statement::Expression(expr_stmt) => {
//             if let Some(ref expression) = expr_stmt.expression {
//                 eval_expression(expression, env)
//             } else {
//                 NULL.clone()
//             }
//         }
//         Statement::Return(return_stmt) => {
//             if let Some(ref return_value) = return_stmt.return_value {
//                 let val = eval_expression(return_value, env);
//                 if is_error(&val) {
//                     return val;
//                 }
//                 Object::ReturnValue(ReturnValue::new(val))
//             } else {
//                 Object::ReturnValue(ReturnValue::new(NULL.clone()))
//             }
//         }
//         Statement::Let(let_stmt) => {
//             if let Some(ref value) = let_stmt.value {
//                 let val = eval_expression(value, env.clone());
//                 if is_error(&val) {
//                     return val;
//                 }
//                 env.borrow_mut()
//                     .set(let_stmt.name.value.clone(), val.clone());
//                 val
//             } else {
//                 NULL.clone()
//             }
//         }
//         Statement::Block(block_stmt) => eval_block_statement(block_stmt, env),
//     }
// }

// pub fn eval_expression(expr: &Expression, env: Rc<RefCell<Environment>>) -> Object {
//     match expr {
//         Expression::IntegerLiteral(int_lit) => Object::Integer(Integer::new(int_lit.value)),
//         Expression::Boolean(boolean) => native_bool_to_boolean_object(boolean.value),
//         Expression::StringLiteral(string_lit) => {
//             Object::String(StringObj::new(string_lit.value.clone()))
//         }
//         Expression::PrefixExpression {
//             operator, right, ..
//         } => {
//             let right_val = eval_expression(right, env);
//             if is_error(&right_val) {
//                 return right_val;
//             }
//             eval_prefix_expression(operator, &right_val)
//         }
//         Expression::InfixExpression {
//             left,
//             operator,
//             right,
//             ..
//         } => {
//             let left_val = eval_expression(left, env.clone());
//             if is_error(&left_val) {
//                 return left_val;
//             }

//             let right_val = eval_expression(right, env);
//             if is_error(&right_val) {
//                 return right_val;
//             }

//             eval_infix_expression(operator, &left_val, &right_val)
//         }
//         Expression::IfExpression(if_expr) => eval_if_expression(if_expr, env),
//         Expression::Identifier(ident) => eval_identifier(ident, env),
//         Expression::FunctionLiteral(fn_lit) => Object::Function(Function::new(
//             fn_lit.parameters.clone(),
//             fn_lit.body.clone(),
//             env,
//         )),
//         Expression::CallExpression(call_expr) => {
//             let function = eval_expression(&call_expr.function, env.clone());
//             if is_error(&function) {
//                 return function;
//             }

//             let args = eval_expressions(&call_expr.arguments, env);
//             if args.len() == 1 && is_error(&args[0]) {
//                 return args[0].clone();
//             }

//             apply_function(&function, &args)
//         }
//         Expression::ArrayLiteral(array_lit) => {
//             let elements = eval_expressions(&array_lit.elements, env);
//             if elements.len() == 1 && is_error(&elements[0]) {
//                 return elements[0].clone();
//             }
//             Object::Array(Array::new(elements))
//         }
//         Expression::IndexExpression(index_expr) => {
//             let left = eval_expression(&index_expr.left, env.clone());
//             if is_error(&left) {
//                 return left;
//             }
//             let index = eval_expression(&index_expr.index, env);
//             if is_error(&index) {
//                 return index;
//             }
//             eval_index_expression(&left, &index)
//         }
//         Expression::HashLiteral(hash_lit) => eval_hash_literal(hash_lit, env),
//     }
// }

// fn eval_program(statements: &[Statement], env: Rc<RefCell<Environment>>) -> Object {
//     let mut result = NULL.clone();

//     for statement in statements {
//         result = eval_statement(statement, env.clone());

//         match &result {
//             Object::ReturnValue(return_val) => return *return_val.value.clone(),
//             Object::Error(_) => return result,
//             _ => {}
//         }
//     }

//     result
// }

// fn eval_block_statement(block: &BlockStatement, env: Rc<RefCell<Environment>>) -> Object {
//     let mut result = NULL.clone();

//     for statement in &block.statements {
//         result = eval_statement(statement, env.clone());

//         match result.object_type() {
//             ObjectType::ReturnValue | ObjectType::Error => return result,
//             _ => {}
//         }
//     }

//     result
// }

// fn native_bool_to_boolean_object(input: bool) -> Object {
//     if input {
//         TRUE.clone()
//     } else {
//         FALSE.clone()
//     }
// }

// fn eval_prefix_expression(operator: &str, right: &Object) -> Object {
//     match operator {
//         "!" => eval_bang_operator_expression(right),
//         "-" => eval_minus_prefix_operator_expression(right),
//         _ => new_error(format!(
//             "unknown operator: {}{}",
//             operator,
//             right.object_type()
//         )),
//     }
// }

// fn eval_infix_expression(operator: &str, left: &Object, right: &Object) -> Object {
//     match (left, right) {
//         (Object::Integer(left_int), Object::Integer(right_int)) => {
//             eval_integer_infix_expression(operator, left_int, right_int)
//         }
//         (Object::String(left_str), Object::String(right_str)) => {
//             eval_string_infix_expression(operator, left_str, right_str)
//         }
//         _ => match operator {
//             "==" => native_bool_to_boolean_object(left == right),
//             "!=" => native_bool_to_boolean_object(left != right),
//             _ => {
//                 if left.object_type() != right.object_type() {
//                     new_error(format!(
//                         "type mismatch: {} {} {}",
//                         left.object_type(),
//                         operator,
//                         right.object_type()
//                     ))
//                 } else {
//                     new_error(format!(
//                         "unknown operator: {} {} {}",
//                         left.object_type(),
//                         operator,
//                         right.object_type()
//                     ))
//                 }
//             }
//         },
//     }
// }

// fn eval_bang_operator_expression(right: &Object) -> Object {
//     match right {
//         Object::Boolean(Boolean { value: true }) => FALSE.clone(),
//         Object::Boolean(Boolean { value: false }) => TRUE.clone(),
//         Object::Null(_) => TRUE.clone(),
//         _ => FALSE.clone(),
//     }
// }

// fn eval_minus_prefix_operator_expression(right: &Object) -> Object {
//     match right {
//         Object::Integer(int) => Object::Integer(Integer::new(-int.value)),
//         _ => new_error(format!("unknown operator: -{}", right.object_type())),
//     }
// }

// fn eval_integer_infix_expression(operator: &str, left: &Integer, right: &Integer) -> Object {
//     let left_val = left.value;
//     let right_val = right.value;

//     match operator {
//         "+" => Object::Integer(Integer::new(left_val + right_val)),
//         "-" => Object::Integer(Integer::new(left_val - right_val)),
//         "*" => Object::Integer(Integer::new(left_val * right_val)),
//         "/" => Object::Integer(Integer::new(left_val / right_val)),
//         "<" => native_bool_to_boolean_object(left_val < right_val),
//         ">" => native_bool_to_boolean_object(left_val > right_val),
//         "==" => native_bool_to_boolean_object(left_val == right_val),
//         "!=" => native_bool_to_boolean_object(left_val != right_val),
//         _ => new_error(format!(
//             "unknown operator: {} {} {}",
//             ObjectType::Integer,
//             operator,
//             ObjectType::Integer
//         )),
//     }
// }

// fn eval_string_infix_expression(operator: &str, left: &StringObj, right: &StringObj) -> Object {
//     if operator != "+" {
//         return new_error(format!(
//             "unknown operator: {} {} {}",
//             ObjectType::String,
//             operator,
//             ObjectType::String
//         ));
//     }

//     Object::String(StringObj::new(format!("{}{}", left.value, right.value)))
// }

// fn eval_if_expression(if_expr: &IfExpression, env: Rc<RefCell<Environment>>) -> Object {
//     let condition = eval_expression(&if_expr.condition, env.clone());
//     if is_error(&condition) {
//         return condition;
//     }

//     if is_truthy(&condition) {
//         eval_block_statement(&if_expr.consequence, env)
//     } else if let Some(ref alternative) = if_expr.alternative {
//         eval_block_statement(alternative, env)
//     } else {
//         NULL.clone()
//     }
// }

// fn eval_identifier(node: &Identifier, env: Rc<RefCell<Environment>>) -> Object {
//     if let Some(val) = env.borrow().get(&node.value) {
//         return val;
//     }

//     if let Some(builtin) = get_builtin(&node.value) {
//         return builtin;
//     }

//     new_error(format!("identifier not found: {}", node.value))
// }

// fn is_truthy(obj: &Object) -> bool {
//     match obj {
//         Object::Null(_) => false,
//         Object::Boolean(Boolean { value: true }) => true,
//         Object::Boolean(Boolean { value: false }) => false,
//         _ => true,
//     }
// }

// fn new_error(message: String) -> Object {
//     Object::Error(Error::new(message))
// }

// fn is_error(obj: &Object) -> bool {
//     matches!(obj.object_type(), ObjectType::Error)
// }

// fn eval_expressions(exps: &[Expression], env: Rc<RefCell<Environment>>) -> Vec<Object> {
//     let mut result = Vec::new();

//     for e in exps {
//         let evaluated = eval_expression(e, env.clone());
//         if is_error(&evaluated) {
//             return vec![evaluated];
//         }
//         result.push(evaluated);
//     }

//     result
// }

// fn apply_function(function: &Object, args: &[Object]) -> Object {
//     match function {
//         Object::Function(func) => {
//             let extended_env = extend_function_env(func, args);
//             let evaluated = eval_block_statement(&func.body, extended_env);
//             unwrap_return_value(evaluated)
//         }
//         Object::Builtin(builtin) => (builtin.function)(args),
//         _ => new_error(format!("not a function: {}", function.object_type())),
//     }
// }

// fn extend_function_env(func: &Function, args: &[Object]) -> Rc<RefCell<Environment>> {
//     let env = Rc::new(RefCell::new(Environment::new_enclosed(func.env.clone())));

//     for (param_idx, param) in func.parameters.iter().enumerate() {
//         if let Some(arg) = args.get(param_idx) {
//             env.borrow_mut().set(param.value.clone(), arg.clone());
//         }
//     }

//     env
// }

// fn unwrap_return_value(obj: Object) -> Object {
//     match obj {
//         Object::ReturnValue(return_val) => *return_val.value,
//         _ => obj,
//     }
// }

// fn eval_index_expression(left: &Object, index: &Object) -> Object {
//     match (left, index) {
//         (Object::Array(array), Object::Integer(int)) => eval_array_index_expression(array, int),
//         (Object::Hash(hash), _) => eval_hash_index_expression(hash, index),
//         _ => new_error(format!(
//             "index operator not supported: {}",
//             left.object_type()
//         )),
//     }
// }

// fn eval_array_index_expression(array: &Array, index: &Integer) -> Object {
//     let idx = index.value;
//     let max = array.elements.len() as i64 - 1;

//     if idx < 0 || idx > max {
//         NULL.clone()
//     } else {
//         array.elements[idx as usize].clone()
//     }
// }

// fn eval_hash_literal(hash_lit: &HashLiteral, env: Rc<RefCell<Environment>>) -> Object {
//     let mut pairs = HashMap::new();

//     for (key_node, value_node) in &hash_lit.pairs {
//         let key = eval_expression(key_node, env.clone());
//         if is_error(&key) {
//             return key;
//         }

//         let hash_key = match key.hash_key() {
//             Ok(hk) => hk,
//             Err(_) => return new_error(format!("unusable as hash key: {}", key.object_type())),
//         };

//         let value = eval_expression(value_node, env.clone());
//         if is_error(&value) {
//             return value;
//         }

//         pairs.insert(hash_key, HashPair { key, value });
//     }

//     Object::Hash(HashObj { pairs })
// }

// // fn eval_hash_index_expression(hash: &HashObj, index: &Object) -> Object {
