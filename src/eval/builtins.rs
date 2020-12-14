use super::{EvalError, EvalResult, Object};

pub type BuiltinFunction = fn(Vec<Object>) -> EvalResult;
pub type Builtin = (&'static str, BuiltinFunction);

macro_rules! builtin {
    ($name:ident) => {
        (stringify!($name), $name)
    };
}

pub static BUILTINS: &[Builtin] = &[
    builtin!(println_str),
    builtin!(println_int),
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

pub fn main(_args: Vec<Object>) -> EvalResult {
    log::debug!("(builtin) eval fn main");
    Ok(Object::Void)
}

pub fn print_str(args: Vec<Object>) -> EvalResult { print(args, "print_str", false) }

pub fn print_int(args: Vec<Object>) -> EvalResult { print(args, "print_int", false) }

pub fn println_str(args: Vec<Object>) -> EvalResult { print(args, "println_str", true) }

pub fn println_int(args: Vec<Object>) -> EvalResult { print(args, "println_int", true) }

pub fn print(args: Vec<Object>, func_name: &str, newline: bool) -> EvalResult {
    log::debug!(
        "(builtin) eval fn {}({})",
        func_name,
        args.iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(",")
    );

    match args.as_slice() {
        [obj] => {
            if newline {
                println!("{}", obj);
            } else {
                print!("{}", obj);
            }
            Ok(Object::Void)
        }

        _ => Err(EvalError::WrongArgumentCount {
            name: func_name.to_owned(),
            expected: 1,
            given: args.len(),
        }),
    }
}
