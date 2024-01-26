pub mod builtins;
pub mod env;
pub mod object;

use std::{fmt, os::raw::c_int};

use log::{debug, trace};
use parser::{self, cstr_to_string, BinOp, Func, FuncDef, Node, NodePtr, YYTokenType};

use self::{
    env::{Env, EnvScope},
    object::Object,
};

// pub type Env = Envv<Object>;
pub type EvalResult = Result<Object, EvalError>;

#[derive(Debug)]
pub enum EvalError {
    Return(Object),
    Redeclaration(String),
    DivisionByZero,
    NotCallable(String),
    AssignToLiteral,
    WrongArgumentCount {
        name: String,
        expected: usize,
        given: usize,
    },
    NotBool(Object),
    UnboundVariable(String),
    UnsupportedInfixOperation(BinOp, Object, Object),
    UnexpectedNodeError {
        ptr: NodePtr,
        expected: YYTokenType,
    },

    /// whether loop should break or continue. if true then break
    LoopControl(bool),
    /// generic parser error
    ParserError,
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::LoopControl(break_or_continue) => write!(
                f,
                "LoopControl {}",
                if *break_or_continue {
                    "break"
                } else {
                    "continue"
                }
            ),
            EvalError::Return(value) => write!(f, "Return: {}", value),
            EvalError::Redeclaration(var) => write!(f, "redeclaration of `{}`", var),
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
            EvalError::UnboundVariable(v) => write!(f, "variable `{}` is not defined.", v),
            EvalError::UnsupportedInfixOperation(op, l, r) => write!(
                f,
                "unsupported operation: {} {} {} ",
                l.type_name(),
                op,
                r.type_name()
            ),
            EvalError::UnexpectedNodeError { ptr, expected } => {
                write!(f, "parse error at {:?} expected {:?}", ptr, expected)
            }
            EvalError::AssignToLiteral => write!(f, "cannot assign to literal"),
            EvalError::ParserError => write!(f, "input could not be parsed"),
        }
    }
}

fn eval_args(param: NodePtr, env: &mut Env) -> Result<Vec<Object>, EvalError> {
    debug!("(eval) evaluating args at {:?}", param);

    parser::parse_args(param)
        .into_iter()
        .inspect(|ptr| trace!("(eval) eval arg at {:?}", ptr))
        .map(|ptr: NodePtr| eval_value(ptr, env))
        .collect()
}

fn eval_ident(ident: String, env: &Env) -> EvalResult {
    let rs = env
        .get(&ident)
        .map(Clone::clone)
        .or_else(|| builtins::lookup(&ident))
        .ok_or(EvalError::UnboundVariable(ident));

    rs
}

fn eval_infix(infix: BinOp, left: Object, right: Object) -> EvalResult {
    match (left, right) {
        (Object::Int(left), Object::Int(right)) => eval_infix_int(&infix, &left, &right),
        (l, r) => Err(EvalError::UnsupportedInfixOperation(infix, l, r)),
    }
}

fn eval_infix_int(infix: &BinOp, left: &c_int, right: &c_int) -> EvalResult {
    match infix {
        BinOp::Add => Ok(Object::Int(left + right)),
        BinOp::Sub => Ok(Object::Int(left - right)),
        BinOp::Mul => Ok(Object::Int(left * right)),
        BinOp::Mod => Ok(Object::Int(left % right)),
        BinOp::Div => {
            if right == &0 {
                Err(EvalError::DivisionByZero)
            } else {
                Ok(Object::Int(left / right))
            }
        }
        BinOp::Less => Ok(Object::Bool(left < right)),
        BinOp::Greater => Ok(Object::Bool(left > right)),
        BinOp::LessEqual => Ok(Object::Bool(left <= right)),
        BinOp::GreaterEqual => Ok(Object::Bool(left >= right)),
        BinOp::Equal => Ok(Object::Bool(left == right)),
        BinOp::NotEqual => Ok(Object::Bool(left != right)),
    }
}

fn eval_while_loop(block: Node, env: &mut Env) -> EvalResult {
    debug!("(eval) start while loop at {:?}", block);

    let condition_node = block.left;
    let block_node = block.right;

    while eval_truthy(condition_node, env)? {
        let rs = eval_block(|env| eval_value(block_node, env), env);

        match rs {
            Err(EvalError::LoopControl(break_or_continue)) => {
                if break_or_continue {
                    break;
                } else {
                    continue;
                }
            }

            _ => {
                rs?;
            }
        };
    }

    Ok(Object::Void)
}

fn eval_truthy(ptr: NodePtr, env: &mut Env) -> Result<bool, EvalError> {
    trace!("(eval::truthy) start at {:?}", ptr);
    let condition = eval_value(ptr, env)?;

    let rs = condition.truthy().ok_or(EvalError::NotBool(condition));

    trace!("(eval::truthy) evaluated {:?} at {:?}", rs, ptr);

    rs
}

fn eval_if_block(node: Node, env: &mut Env) -> EvalResult {
    let truthy = eval_truthy(node.left, env)?;

    if let Some((block, Some(token))) = Node::deref_node_and_token(node.right) {
        match (token, truthy) {
            (YYTokenType::ELSE, boolean) => eval_block(
                |env| eval_value(if boolean { block.left } else { block.right }, env),
                env,
            ),
            (_, true) => eval_block(|env| eval_value(node.right, env), env),
            _ => Ok(Object::Void),
        }
    } else {
        Err(EvalError::UnexpectedNodeError {
            ptr: node.right,
            expected: YYTokenType::d,
        })
    }
}

fn eval_block<B>(block_fnc: B, env: &mut Env) -> EvalResult
where
    B: FnOnce(&mut Env) -> EvalResult,
{
    eval_block_scope(block_fnc, env.current_scope(), env)
}

fn eval_block_scope<B>(block_fnc: B, scope: EnvScope, env: &mut Env) -> EvalResult
where
    B: FnOnce(&mut Env) -> EvalResult,
{
    debug!("(eval) eval_block_scope {}", scope);
    trace!("(eval) call env: {:#?}", env);

    env.extend_scope(scope, |env| block_fnc(env))
}

fn eval_fn(closure: Func, scope: EnvScope, args: Vec<Object>, env: &mut Env) -> EvalResult {
    trace!("(eval) {} {:?} with {:?}", scope, closure, args);

    eval_block_scope(
        move |block_env| {
            closure
                .def
                .parameters
                .into_iter()
                .zip(args.into_iter())
                .for_each(|(param, arg)| {
                    block_env.declare(param, arg);
                });

            let head = closure.head;
            let result = eval_value(head, block_env);

            if let Err(EvalError::Return(rs)) = result {
                Ok(rs)
            } else {
                result
            }
        },
        scope,
        env,
    )
}

pub fn eval_fn_call(func: Object, args: Vec<Object>, env: &mut Env) -> EvalResult {
    debug!("(eval) calling fn {} with {:?}", func, args);

    match func {
        Object::Closure(scope, closure) => eval_fn(closure, scope, args, env),
        Object::Function(fnc) => eval_fn(fnc, env::GLOBAL_SCOPE, args, env),
        Object::Str(ident) | Object::Ident(ident) => match eval_ident(ident, env)? {
            Object::Function(fnc) => eval_fn(fnc, env::GLOBAL_SCOPE, args, env),
            Object::Closure(scope, fnc) => eval_fn(fnc, scope, args, env),
            Object::BuiltinFunction(builtin) => builtins::eval(builtin, args),
            obj => Err(EvalError::NotCallable(obj.type_name())),
        },

        obj => Err(EvalError::NotCallable(obj.type_name())),
    }
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

pub fn eval_tree(ptr: NodePtr, env: &mut Env) -> EvalResult {
    if let Some((node, Some(token))) = Node::deref_node_and_token(ptr) {
        trace!("(eval) walking {:?} ({}) at {:?}", token, node.type_, ptr,);

        match token {
            YYTokenType::LEAF => eval_tree(node.left, env),

            YYTokenType::RETURN => Err(EvalError::Return(eval_value(node.left, env)?)),

            YYTokenType::CONSTANT => Ok(Object::Int(node.as_cint().unwrap())),

            YYTokenType::StringLiteral => Ok(Object::Str(unsafe {
                cstr_to_string(node.as_cstr().unwrap())
            })),

            YYTokenType::IDENTIFIER => Ok(Object::Ident(unsafe {
                cstr_to_string(node.as_cstr().unwrap())
            })),

            YYTokenType::IF => eval_if_block(node, env),

            // left child is the loop condition; right child is a statement orsequence of statements
            YYTokenType::WHILE => eval_while_loop(node, env),

            // evaluate linked statements in order of appearance
            YYTokenType::LinkedNodeBlock => {
                eval_value(node.left, env).and_then(|_| eval_value(node.right, env))
            }

            // D: left is the function info (d), right is the function
            YYTokenType::D => match node.left_node_and_token() {
                Some((def, YYTokenType::d)) => {
                    let return_type = def.left_node().and_then(Node::as_string).unwrap();
                    let (name, parameters) = parser::parse_fn(def.right_node().unwrap());

                    let scope = env.current_scope();
                    let fnc = Func {
                        head: node.right,
                        def: FuncDef {
                            name: name.clone(),
                            return_type,
                            parameters,
                        },
                    };

                    let fnc = if scope == env::GLOBAL_SCOPE {
                        Object::Function(fnc)
                    } else {
                        Object::Closure(scope, fnc)
                    };

                    env.declare(name, fnc.clone());
                    Ok(fnc)
                }
                _ => Err(EvalError::UnexpectedNodeError {
                    ptr: node.left,
                    expected: YYTokenType::d,
                }),
            },

            YYTokenType::Infix(infix) => eval_infix(
                infix,
                eval_value(node.left, env)?,
                eval_value(node.right, env)?,
            ),

            YYTokenType::APPLY => {
                let func_name = eval_tree(node.left, env)?;
                let args = eval_args(node.right, env)?;

                eval_fn_call(func_name, args, env)
            }

            // A variable or a function declaration (~)
            // In the case of a variable, the leftchild is the type and right child
            // is the variable (or list of variables) to bedeclared. In the case of
            // a function, the right child is an AST holding therest of the function text.
            YYTokenType::DECLARE => {
                if let Some(left_node_type) = node.left_node_token() {
                    match left_node_type {
                        YYTokenType::LEAF | YYTokenType::INT => env.declare_next(),
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

                    env.set(name, default_value.clone())?;
                    env.undeclare_next();

                    Ok(default_value)
                } else {
                    Ok(decl)
                }
            }

            YYTokenType::ASSIGN => {
                let name = eval_tree(node.left, env)?;
                let value = eval_tree(node.right, env)?;

                trace!("assigning: name = {:?}, value = {:?}", name, value);

                if let Object::Ident(name) = name {
                    env.set(name, value.clone())?;
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

            YYTokenType::BREAK => Err(EvalError::LoopControl(true)),
            YYTokenType::CONTINUE => Err(EvalError::LoopControl(false)),

            _t => {
                println!("tried to eval node with type {}", node.type_);
                Ok(Object::Void)
            }
        }
    } else {
        Ok(Object::Void)
    }
}

pub fn eval_repl(input: &str, env: &mut Env) -> EvalResult {
    let input = format!("void __repl__() {{ {} }}", input);
    if let Some(ast) = parser::parse_str(input.as_str()) {
        let obj = eval_tree(ast, env)?;

        eval_fn_call(obj, vec![], env)
    } else {
        Err(EvalError::ParserError)
    }
}

pub fn eval_prog(ast_root: NodePtr) -> EvalResult {
    let env: &mut Env = &mut Env::new();

    eval_tree(ast_root, env)
        .and_then(|_| eval_fn_call(Object::Ident("main".to_owned()), vec![], env))
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
        // let mut errors = Vec::new();

        // tests.iter().map(a)
        // for (input, expected) in &tests {
        //     match eval_input(input) {
        //         Ok(obj) => {
        //             if obj.to_string().ne(&expected.to_string()) {
        //                 errors.push((input, expected, None))
        //             }
        //         }
        //         Err(err) => errors.push((input, expected)),
        //     }
        // }

        // for (input, expected) in errors
        //             debug!(
        //                 "expected `{}`, but got error=`{}` for `{}`",
        //                 expected, err, input
        //             );
    }

    fn expect_errors(tests: Vec<(&str, &str)>) {
        for (input, expected_message) in &tests {
            match eval_input(input) {
                Ok(obj) => {
                    debug!("no error object returned. got=`{}` for `{}`", obj, input);
                }
                Err(err) => {
                    // assert_eq!(&err.to_string(), expected_message, "for `{}`", input);
                }
            }
        }
    }

    fn eval_input(input: &str) -> EvalResult { eval_repl(input, &mut Env::new()) }
}
