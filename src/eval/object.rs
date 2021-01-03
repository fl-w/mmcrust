use parser::Func;
use std::{fmt, os::raw::c_int};

use super::{builtins::Builtin, env::EnvScope};

#[derive(PartialEq, Clone)]
pub enum Object {
    Int(c_int),
    Bool(bool),
    Str(String),
    Ident(String),
    Function(Func),
    Closure(EnvScope, Func),
    BuiltinFunction(Builtin),
    Void,
}

impl Object {
    pub fn type_name(&self) -> String {
        match self {
            Self::Int(_) => "int",
            Self::Str(_) => "str",
            Self::Ident(_) => "var",
            Self::Void => "void",
            Self::Bool(_) => "bool",
            Self::Closure(_, _) => "function <closure>",
            Self::BuiltinFunction(_) => "built-in",
            Self::Function(_) => "function",
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
            Object::Function(fnc) => {
                write!(
                    f,
                    "<{} {} at {:?}>",
                    self.type_name(),
                    fnc.def.name,
                    fnc.head
                )
            }
            Object::Closure(scope, fnc) => {
                write!(f, "<{} at {:?}::{}>", self.type_name(), fnc.head, scope)
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
