pub mod builtins;
pub mod env;

use std::{
    convert::TryInto,
    ffi::CStr,
    fmt,
    os::raw::{c_char, c_int},
};

use log::trace;
use parser::{self, Infix, Node, NodePtr, YYTokenType};

use self::env::Env;
use self::{builtins::Builtin, env::EnvScope};

#[derive(PartialEq, Clone)]
pub enum Object {
    Int(c_int),
    Bool(bool),
    Str(String),
    Ident(String),
    Closure(Closure),
    Function(String, Closure),
    BuiltinFunction(Builtin),
    Void,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Closure {
    head: NodePtr,
    scope: EnvScope,
    return_type: String,
    parameters: Vec<String>,
}

pub type TypeName = String;

impl Object {
    pub fn type_name(&self) -> TypeName {
        match self {
            Self::Int(_) => "int",
            Self::Str(_) => "str",
            Self::Ident(_) => "var",
            Self::Void => "void",
            Self::Bool(_) => "bool",
            Self::Closure(_) => "function <closure>",
            Self::BuiltinFunction(_) => "built-in",
            Self::Function(_, _) => "function",
        }
        .to_owned()
    }

    pub fn truthy(&self) -> Option<bool> {
        match self {
            Self::Void => Some(false),
            Self::Int(i) => Some(i != &0),
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Str(string) => write!(f, "\"{}\"", string),
            Object::BuiltinFunction((name, _)) => write!(f, "<{}>", name),
            Object::Ident(value) => write!(f, "<ident {}>", value),
            Object::Function(name, cl) => {
                write!(f, "<{} {} at {:?}>", self.type_name(), name, cl.head)
            }
            Object::Closure(cl) => {
                write!(f, "<{} at {:?}>", self.type_name(), cl.head)
            }

            _ => write!(f, "{}", self),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Int(value) => write!(f, "{}", value),
            Object::Bool(value) => write!(f, "{}", value),
            Object::Str(value) => write!(f, "{}", value),
            Object::Void => write!(f, "void"),
            _ => write!(f, "{:?}", self),
        }
    }
}

// impl From<YYTokenType> for Object {
//     fn from(token_type: YYTokenType) -> Self {
//         match token_type {
//             _ => Object::Int(0),
//         }
//     }
// }

pub type EvalResult = Result<Object, EvalError>;

pub enum EvalError {
    Return(Object),
    DivisionByZero,
    NotCallable(String),
    AssignToLiteral,
    WrongArgumentCount {
        name: String,
        expected: usize,
        given: usize,
    },
    TypeMismatch {
        expected: TypeName,
        given: TypeName,
    },
    NotBool(Object),
    UnboundVariable(String),
    UnsupportedInfixOperation(Infix, Object, Object),
    ParseError(i32),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::Return(value) => write!(f, "Return: {}", value),
            EvalError::DivisionByZero => write!(f, "DivisionByZero: zero division"),
            EvalError::NotCallable(type_name) => {
                write!(f, "'{}' object is not callable.", type_name)
            }
            EvalError::NotBool(v) => write!(f, "{} is not a valid bool", v),
            EvalError::WrongArgumentCount {
                name,
                expected,
                given,
            } => write!(
                f,
                "{}() takes {} arguments ({} given)",
                name, expected, given
            ),
            EvalError::TypeMismatch { expected, given } => {
                write!(f, "type mismatch: {}, {}", expected, given)
            }

            EvalError::UnboundVariable(v) => write!(f, "variable '{}' is not defined.", v),
            EvalError::UnsupportedInfixOperation(op, l, r) => write!(
                f,
                "unsupported operation: {} {} {} ",
                l.type_name(),
                op,
                r.type_name()
            ),
            EvalError::ParseError(n) => {
                write!(f, "pleaase fix: try to eval node with .type_ = {}", n)
            }
            EvalError::AssignToLiteral => write!(f, "cannot assign to literal"),
        }
    }
}

fn cstr_to_string(ptr: *const c_char) -> String {
    unsafe { CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string() }
}

fn eval_args(param: NodePtr, env: &mut Env) -> Result<Vec<Object>, EvalError> {
    let mut parameter_list = vec![];
    let mut param = param;

    while !parser::node_ptr_null(param) {
        let node = unsafe { param.as_mut().unwrap() };

        if let YYTokenType::Comma = node.type_.try_into().unwrap() {
            parameter_list.push(eval_value(node.left, env)?);
            param = node.right;
        } else {
            parameter_list.push(eval_value(param, env)?);
            break;
        };
    }

    Ok(parameter_list)
}

fn eval_conditional(node: Node, env: &mut Env) -> EvalResult {
    let condition = eval_value(node.left, env)?;
    let block: Node = node.right_node().unwrap();

    let token_type = block.token_type().unwrap();
    let truthy = condition.truthy().ok_or(EvalError::NotBool(condition))?;

    match (token_type, truthy) {
        (YYTokenType::ELSE, true) => eval_return_tree(block.left, env),
        (YYTokenType::ELSE, false) => eval_return_tree(block.right, env),
        (_, true) => eval_return_tree(node.right, env),
        _ => Ok(Object::Void),
    }
}

pub fn eval_function_call(func: Object, args: Vec<Object>, env: &mut Env) -> EvalResult {
    trace!("calling function with name = {}, args = {:?}", func, args);

    match func {
        Object::Closure(closure) => eval_closure(closure, args, env),
        Object::Str(ident) | Object::Ident(ident) => match eval_ident(ident, env)? {
            Object::Function(_, closure) | Object::Closure(closure) => {
                eval_closure(closure, args, env)
            }
            Object::BuiltinFunction(builtin) => builtins::eval(builtin, args),
            obj => Err(EvalError::NotCallable(obj.type_name())),
        },

        obj => Err(EvalError::NotCallable(obj.type_name())),
    }
}

fn eval_ident(ident: String, env: &Env) -> EvalResult {
    env.get(&ident)
        .map(Clone::clone)
        .or_else(|| builtins::lookup(&ident))
        .ok_or(EvalError::UnboundVariable(ident))
}

fn eval_closure(closure: Closure, args: Vec<Object>, env: &mut Env) -> EvalResult {
    env.extend(closure.scope);

    for (param, arg) in closure.parameters.into_iter().zip(args.into_iter()) {
        env.declare_next();
        env.set(param, arg);
    }

    let rs = eval_return_tree(closure.head, &mut env);

    env.drop_frame();

    rs
}

fn eval_infix(infix: Infix, left: Object, right: Object) -> EvalResult {
    match (left, right) {
        (Object::Int(left), Object::Int(right)) => eval_infix_int(&infix, &left, &right),
        (l, r) => Err(EvalError::UnsupportedInfixOperation(infix, l, r)),
    }
}

fn eval_infix_int(infix: &Infix, left: &c_int, right: &c_int) -> EvalResult {
    match infix {
        Infix::Add => Ok(Object::Int(left + right)),
        Infix::Subtract => Ok(Object::Int(left - right)),
        Infix::Multiply => Ok(Object::Int(left * right)),
        Infix::Divide => {
            if right == &0 {
                Err(EvalError::DivisionByZero)
            } else {
                Ok(Object::Int(left / right))
            }
        }
        Infix::Less => Ok(Object::Bool(left < right)),
        Infix::Greater => Ok(Object::Bool(left > right)),
        Infix::LessEqual => Ok(Object::Bool(left <= right)),
        Infix::GreaterEqual => Ok(Object::Bool(left >= right)),
        Infix::Equal => Ok(Object::Bool(left == right)),
        Infix::NotEqual => Ok(Object::Bool(left != right)),
    }
}

fn eval_block(block: Node, env: &mut Env) -> EvalResult {
    eval_return_tree(block.left, env)?;
    eval_return_tree(block.right, env) // temporary?
}

/// Given a function definition node, return the function name and list of parameter names.
fn parse_function_def(node: Node) -> (String, Vec<String>) {
    let parse_parameter = |pnode: Node| match pnode.token_type() {
        Some(YYTokenType::LEAF) => pnode.as_string().unwrap(),
        Some(YYTokenType::Declaration) => pnode.right_node().unwrap().as_string().unwrap(),
        _ => unreachable!(),
    };

    let mut parameter_list = vec![];
    let mut parameter_node = node.right_node();

    // node F: left child is the name of the function, right is the function parameters
    let function_name = node.left_node().and_then(|node| node.as_string()).unwrap();
    while let Some(pnode) = parameter_node {
        match pnode.token_type() {
            Some(YYTokenType::Comma) => {
                parameter_list.insert(0, parse_parameter(pnode.right_node().unwrap()));
                parameter_node = pnode.left_node();
            }

            Some(_) => {
                parameter_list.insert(0, parse_parameter(pnode));
                break;
            }

            None => break,
        }
    }

    (
        function_name,
        parameter_list.drain_filter(|s| !s.is_empty()).collect(),
    )
}

fn eval_return_tree(tree: NodePtr, env: &mut Env) -> EvalResult {
    let result = eval_value(tree, env);

    if let Err(EvalError::Return(v)) = result {
        return Ok(v);
    }

    result
}

pub fn eval_value(node: NodePtr, env: &mut Env) -> EvalResult {
    match eval_tree(node, env)? {
        Object::Ident(name) => match &name[..] {
            "true" => Ok(Object::Bool(true)),
            "false" => Ok(Object::Bool(false)),
            _ => env
                .get(&name)
                .map(Clone::clone)
                .ok_or(EvalError::UnboundVariable(name)),
        },
        _t => Ok(_t),
    }
}

fn _trace_tree(tree: NodePtr, node: Node) {
    let name = unsafe { CStr::from_ptr(parser::named(node.type_)) }
        .to_str()
        .unwrap();
    trace!(
        "eval ({})> {} :: ({}, {}, {})",
        node.type_,
        name,
        parser::node_ptr_null(tree),
        parser::node_ptr_null(node.left),
        parser::node_ptr_null(node.right),
    );
}

pub fn eval_tree(tree: NodePtr, env: &mut Env) -> EvalResult {
    if let Ok(node) = tree.try_into() as Result<Node, ()> {
        _trace_tree(tree, node);

        match node
            .token_type()
            .ok_or(EvalError::ParseError(node.type_ as i32))? as YYTokenType
        {
            YYTokenType::LEAF => eval_tree(node.left, env),

            YYTokenType::RETURN => Err(EvalError::Return(eval_value(node.left, env)?)),

            YYTokenType::CONSTANT => Ok(Object::Int(node.as_cint().unwrap())),

            YYTokenType::STRING_LITERAL => Ok(Object::Str(cstr_to_string(node.as_cstr().unwrap()))),

            YYTokenType::IDENTIFIER => Ok(Object::Ident(cstr_to_string(node.as_cstr().unwrap()))),

            YYTokenType::IF => eval_conditional(node, env),

            // evaluated linked statements in order of appearance
            YYTokenType::LinkedNodeBlock => eval_block(node, env),

            // D: left is the function info (d), right is the function
            YYTokenType::D => match node.left_node_and_token() {
                Some((def, YYTokenType::d)) => {
                    let return_type = def.left_node().and_then(Node::as_string).unwrap();
                    let (function_name, parameters) = parse_function_def(def.right_node().unwrap());

                    let closure = Closure {
                        head: node.right,
                        scope: env.current_scope(),
                        return_type,
                        parameters,
                    };

                    env.set(
                        function_name.clone(),
                        Object::Function(function_name, closure),
                    );

                    Ok(Object::Void)
                }
                _ => Err(EvalError::ParseError(2)),
            },

            YYTokenType::Infix(infix) => eval_infix(
                infix,
                eval_value(node.left, env)?,
                eval_value(node.right, env)?,
            ),

            YYTokenType::APPLY => {
                let func_name = eval_tree(node.left, env)?;
                let args = eval_args(node.right, env)?;

                eval_function_call(func_name, args, env)
            }

            // A variable or a function declaration (~)
            // In the case of a variable, the leftchild is the type and right child
            // is the variable (or list of variables) to bedeclared. In the case of
            // a function, the right child is an AST holding therest of the function text.
            YYTokenType::Declaration => {
                if let Some(left_node_type) = node.left_node().and_then(|n| n.token_type()) {
                    match left_node_type {
                        YYTokenType::LEAF | YYTokenType::INT => {}
                        _ => {
                            eval_tree(node.left, env)?;
                        }
                    }
                };

                let decl = eval_tree(node.right, env)?;
                if let Object::Ident(name) = decl {
                    // declaration with no assignment.
                    // TODO: change this
                    // let left_node = (node.left.try_into() as Result<Node, ()>)
                    //     .map_err(|_| EvalError::ParseError(0))?;
                    // let declaration_type = left_node.token_type().unwrap();
                    // let default_value: Object = declaration_type.into();
                    let default_value = Object::Int(0);

                    // add to environment
                    env.set(name, default_value.clone());

                    Ok(default_value)
                } else {
                    Ok(decl)
                }
            }

            YYTokenType::Assign => {
                let name = eval_tree(node.left, env)?;
                let value = eval_tree(node.right, env)?;

                trace!("assigning: name = {:?}, value = {:?}", name, value);

                if let Object::Ident(name) = name {
                    env.set(name, value.clone());
                    Ok(value)
                } else {
                    Err(EvalError::AssignToLiteral)
                }
            }

            YYTokenType::UnaryOp => {
                if let Some(val) = node.as_cint() {
                    println!(
                        "bug in parser, for now treat unary operator with {} as '-'",
                        std::char::from_u32(val as u32).unwrap()
                    );
                }
                eval_tree(node.left, env)
            }

            _t => {
                println!("tried to eval node with type {}", node.type_);
                Ok(Object::Void)
            }
        }
    } else {
        // println!("END OF TREE");
        Ok(Object::Void)
    }
}

pub fn eval_repl(input: &str, env: &mut Env) -> EvalResult {
    let input = format!("fuction __repl__() {{ {} }}", input);
    let ast = parser::parse_str(input.as_str()).unwrap();
    let obj = eval_tree(ast, env)?;

    eval_function_call(obj, vec![], env)
}

pub fn eval_source_tree(tree_root: NodePtr) -> EvalResult {
    let env = &mut Env::new();

    eval_tree(tree_root, env)
        .and_then(|_| eval_function_call(Object::Ident("main".to_owned()), vec![], env))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eval_boolean() {
        expect_values(vec![
            // Prefix
            ("!true;", "false"),
            ("!!true;", "true"),
            ("!false;", "true"),
            ("!!false;", "false"),
            ("!void;", "true"),
            ("!!void;", "false"),
            ("!0;", "false"),
            ("1;", "true"),
            ("!3;", "false"),
            ("!!3;", "true"),
            // Infix
            // boolean -> boolean
            ("true == true;", "true"),
            ("false == true;", "false"),
            ("true != true;", "false"),
            ("true != false;", "true"),
            // integer -> boolean
            ("1 == 2;", "false"),
            ("2 == 2;", "true"),
            ("1 != 2;", "true"),
            ("2 != 2;", "false"),
            ("1 > 2;", "false"),
            ("1 < 2;", "true"),
        ]);
    }

    #[test]
    fn eval_integer() {
        expect_values(vec![
            // Prefix
            ("0-123;", "-123"),
            ("0-(-123);", "123"),
            ("0-(3 * 3);", "-9"),
            // Infix
            ("2 + 3;", "5"),
            ("2 - 3;", "-1"),
            ("2 * 3;", "6"),
            ("9 / 3;", "3"),
            ("(0-50) + 100 + (0-50);", "0"),
            ("20 + 2 * (-10);", "0"),
            ("50 / 2 * 2 + 10;", "60"),
            ("2 * (5 + 10);", "30"),
            ("3 * 3 * 3 + 10;", "37"),
            ("3 * (3 * 3) + 10;", "37"),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10;", "50"),
        ]);
    }

    #[test]
    fn eval_if() {
        expect_values(vec![
            ("if (true) { return 10; }", "10;"),
            ("if (false) { 10; }", "void"),
            ("if (void) { 1; } else { 2; }", "2"),
            ("if (2 > 1) { 3; } else { 4; }", "3"),
            ("if (2 < 1) { 3; } else { 4; }", "4"),
            ("if (1 < 2) { 3; }", "3"),
            ("if (1 > 2) { 3; }", "void"),
        ]);
    }

    #[test]
    fn eval_return() {
        expect_values(vec![
            ("return;", "void"),
            ("return 10;", "10"),
            ("1 + 2; return; 3 + 4", "void"),
            ("1 + 2; return 8; 3 + 4", "8"),
            ("3; return 8 * 2; 3 + 4", "16"),
            // Nested statements
            (
                "if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }
                return 1;
            }",
                "10",
            ),
        ]);
    }

    fn expect_values(tests: Vec<(&str, &str)>) {
        for (input, expected) in &tests {
            match eval_input(input) {
                Ok(obj) => {
                    assert_eq!(obj.to_string(), expected.to_string(), "for `{}`", input);
                }
                Err(err) => {
                    panic!(
                        "expected `{}`, but got error=`{}` for `{}`",
                        expected, err, input
                    );
                }
            }
        }
    }

    fn expect_errors(tests: Vec<(&str, &str)>) {
        for (input, expected_message) in &tests {
            match eval_input(input) {
                Ok(obj) => {
                    panic!("no error object returned. got=`{}` for `{}`", obj, input);
                }
                Err(err) => {
                    assert_eq!(&err.to_string(), expected_message, "for `{}`", input);
                }
            }
        }
    }

    fn eval_input(input: &str) -> EvalResult { eval_repl(input, &mut Env::new()) }
}
