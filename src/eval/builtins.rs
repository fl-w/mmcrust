use super::{EvalError, EvalResult, Object};

pub type BuiltinFunction = fn(Vec<Object>) -> EvalResult;
pub type Builtin = (&'static str, BuiltinFunction);

macro_rules! builtin {
    ($name:ident) => {
        (stringify!($name), $name)
    };
}

pub static BUILTINS: &[Builtin] = &[
    builtin!(print_str),
    builtin!(print_int),
    builtin!(len),
    builtin!(main),
];

pub fn lookup(func_name: &str) -> Option<Object> {
    BUILTINS.iter().find_map(|builtin| {
        func_name
            .eq(builtin.0)
            .then_some(Object::BuiltinFunction(*builtin))
    })
}

pub fn eval(builtin: Builtin, args: Vec<Object>) -> EvalResult { builtin.1(args) }

pub fn len(args: Vec<Object>) -> EvalResult {
    match args.as_slice() {
        [Object::Str(string)] => Ok(Object::Int(string.len() as i32)),
        _ => Err(EvalError::WrongArgumentCount {
            name: "len".to_owned(),
            expected: 1,
            given: args.len(),
        }),
    }
}

pub fn main(_args: Vec<Object>) -> EvalResult { Ok(Object::Void) }

pub fn print_str(args: Vec<Object>) -> EvalResult { print(args, "print_str") }

pub fn print_int(args: Vec<Object>) -> EvalResult { print(args, "print_int") }

pub fn print(args: Vec<Object>, func_name: &str) -> EvalResult {
    match args.as_slice() {
        [Object::Str(val)] => {
            print!("{}", val);
            Ok(Object::Void)
        }

        [Object::Int(val)] => {
            print!("{}", val);
            Ok(Object::Void)
        }

        _ => Err(EvalError::WrongArgumentCount {
            name: func_name.to_owned(),
            expected: 1,
            given: args.len(),
        }),
    }
}
