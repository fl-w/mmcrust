use std::{convert::TryInto, ffi::CStr, fmt, os::raw::c_int};

use itertools::Itertools;
use log::{debug, trace};
use parser::{self, cstr_to_string, Infix, Node, NodePtr, YYTokenType};

use self::env::Env;
use self::{builtins::Builtin, env::EnvScope};

#[derive(PartialEq, Clone)]
pub enum Object {
    Int(c_int),
    Bool(bool),
    Str(String),
    Ident(String),
    Function(CompiledFunction),
    Closure(EnvScope, CompiledFunction),
    BuiltinFunction(Builtin),
    Void,
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
                write!(f, "<{} {} at {:?}>", self.type_name(), fnc.name, fnc.head)
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

// impl From<YYTokenType> for Object {
//     fn from(token_type: YYTokenType) -> Self {
//         match token_type {
//             _ => Object::Int(0),
//         }
//     }
// }
