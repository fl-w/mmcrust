#![allow(
    dead_code,
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    clippy::missing_safety_doc,
    improper_ctypes
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::{
    convert::{TryFrom, TryInto},
    ffi::{CStr, CString},
    fmt,
    os::raw::{c_char, c_int},
};

pub type Node = NODE;
pub type NodePtr = *mut Node;

impl Node {
    fn ptr(self) -> NodePtr { Box::into_raw(Box::new(self)) }

    pub fn as_cstr(self) -> Option<*mut c_char> {
        (unsafe { (self.ptr() as TokenPtr).as_mut() }).map(|t| t.lexeme)
        // match self.type_ as u32 {
        //     STRING_LITERAL | IDENTIFIER => {
        //         (unsafe { (self.ptr() as TokenPtr).as_mut() }).map(|t| t.lexeme)
        //     }
        //     _ => None,
        // }
    }

    pub fn as_string(self) -> Option<String> {
        if self
            .token_type()
            .map(|token| matches!(token, YYTokenType::LEAF))
            .unwrap_or(false)
        {
            self.left_node().unwrap()
        } else {
            self
        }
        .as_cstr()
        .map(|ptr| unsafe { CStr::from_ptr(ptr).to_string_lossy().to_owned().to_string() })
    }

    pub fn as_cint(self) -> Option<c_int> {
        // if self.type_ as u32 == CONSTANT {
        (unsafe { (self.ptr() as TokenPtr).as_mut() }).map(|tok| tok.value)
        // } else {
        //     None
        // }
    }

    pub fn token_type(&self) -> Option<YYTokenType> {
        self.type_.try_into().ok() as Option<YYTokenType>
    }

    pub fn left_node(&self) -> Option<Self> { self.left.try_into().ok() }

    pub fn left_node_and_token(&self) -> Option<(Self, YYTokenType)> {
        self.left
            .try_into()
            .ok()
            .and_then(|node: Node| node.token_type().map(|token| (node, token)))
    }

    pub fn left_node_token(&self) -> Option<YYTokenType> {
        self.left_node().and_then(|node| node.token_type())
    }

    pub fn right_node(&self) -> Option<Self> { self.right.try_into().ok() }

    pub fn deref_node_and_token(ptr: NodePtr) -> Option<(Self, Option<YYTokenType>)> {
        let node: Result<Node, ()> = ptr.try_into();

        node.ok().map(|node| (node, node.token_type()))
    }
}

impl TryFrom<NodePtr> for Node {
    type Error = ();

    fn try_from(ptr: NodePtr) -> Result<Self, Self::Error> {
        if node_ptr_null(ptr) {
            Err(())
        } else {
            unsafe { ptr.as_mut() }.map(|ptr| *ptr).ok_or(())
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            type_: 0,
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
        }
    }
}

pub type Token = TOKEN;
pub type TokenPtr = *mut Token;

pub unsafe fn parse_stdin() -> Option<NodePtr> { parse() }

pub fn parse_str(input: &str) -> Option<NodePtr> {
    unsafe {
        let string = CString::new(input).unwrap();
        let buf = yy_scan_string(string.as_ptr());

        let ptr = parse();

        yy_delete_buffer(buf);
        ptr
    }
}

pub fn parse() -> Option<NodePtr> {
    unsafe {
        // init symbol table before parsing
        init_symbtable();

        let i = yyparse();

        if log::log_enabled!(log::Level::Trace) {
            yydebug = 1;
        }

        if i == 0 {
            if log::log_enabled!(log::Level::Debug) {
                print_tree(ans);
            }

            Some(ans)
        } else {
            None
        }
    }
}

pub fn node_ptr_null(ptr: NodePtr) -> bool { unsafe { node_is_null(ptr) == 1 || ptr.is_null() } }

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Equal,
    NotEqual,
}

use BinOp::*;

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Add => "+",
                Sub => "-",
                Mul => "*",
                Div => "/",
                Less => "<",
                Greater => ">",
                LessEqual => "<=",
                GreaterEqual => ">=",
                Equal => "==",
                NotEqual => "!=",
            }
        )
    }
}

#[derive(Debug)]
pub enum YYTokenType {
    IDENTIFIER,
    CONSTANT,
    STRING_LITERAL,
    EXTERN,
    AUTO,
    INT,
    VOID,
    FUNCTION,
    APPLY,
    LEAF,
    IF,
    ELSE,
    WHILE,
    CONTINUE,
    BREAK,
    RETURN,
    D,
    d,
    FunctionDef,
    DECLARE,
    LinkedNodeBlock,
    ASSIGN,
    Comma,
    UnaryOp,
    Infix(BinOp),
}

impl TryFrom<i32> for YYTokenType {
    type Error = i32;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == IDENTIFIER as i32 => Ok(YYTokenType::IDENTIFIER),
            x if x == CONSTANT as i32 => Ok(YYTokenType::CONSTANT),
            x if x == STRING_LITERAL as i32 => Ok(YYTokenType::STRING_LITERAL),
            x if x == EXTERN as i32 => Ok(YYTokenType::EXTERN),
            x if x == AUTO as i32 => Ok(YYTokenType::AUTO),
            x if x == INT as i32 => Ok(YYTokenType::INT),
            x if x == VOID as i32 => Ok(YYTokenType::VOID),
            x if x == FUNCTION as i32 => Ok(YYTokenType::FUNCTION),
            x if x == APPLY as i32 => Ok(YYTokenType::APPLY),
            x if x == LEAF as i32 => Ok(YYTokenType::LEAF),
            x if x == IF as i32 => Ok(YYTokenType::IF),
            x if x == ELSE as i32 => Ok(YYTokenType::ELSE),
            x if x == WHILE as i32 => Ok(YYTokenType::WHILE),
            x if x == CONTINUE as i32 => Ok(YYTokenType::CONTINUE),
            x if x == BREAK as i32 => Ok(YYTokenType::BREAK),
            x if x == RETURN as i32 => Ok(YYTokenType::RETURN),
            x if x == 'd' as i32 => Ok(YYTokenType::d),
            x if x == 'D' as i32 => Ok(YYTokenType::D),
            x if x == 'F' as i32 => Ok(YYTokenType::FunctionDef),
            x if x == '=' as i32 => Ok(YYTokenType::ASSIGN),
            x if x == '~' as i32 => Ok(YYTokenType::DECLARE),
            x if x == ';' as i32 => Ok(YYTokenType::LinkedNodeBlock),
            x if x == '+' as i32 => Ok(YYTokenType::Infix(Add)),
            x if x == '-' as i32 => Ok(YYTokenType::Infix(Sub)),
            x if x == '*' as i32 => Ok(YYTokenType::Infix(Mul)),
            x if x == '/' as i32 => Ok(YYTokenType::Infix(Div)),
            x if x == '>' as i32 => Ok(YYTokenType::Infix(Greater)),
            x if x == '<' as i32 => Ok(YYTokenType::Infix(Less)),
            x if x == LE_OP as i32 => Ok(YYTokenType::Infix(LessEqual)),
            x if x == GE_OP as i32 => Ok(YYTokenType::Infix(GreaterEqual)),
            x if x == EQ_OP as i32 => Ok(YYTokenType::Infix(Equal)),
            x if x == NE_OP as i32 => Ok(YYTokenType::Infix(NotEqual)),
            x if x == ',' as i32 => Ok(YYTokenType::Comma),
            0 => Ok(YYTokenType::UnaryOp),
            _ => Err(v),
        }
    }
}

impl TryFrom<NodePtr> for YYTokenType {
    type Error = ();

    fn try_from(ptr: NodePtr) -> Result<Self, Self::Error> {
        if let Ok(node) = ptr.try_into() as Result<Node, ()> {
            node.token_type().ok_or(())
        } else {
            Err(())
        }
    }
}

pub unsafe fn cstr_to_string(ptr: *const c_char) -> String {
    let string = CStr::from_ptr(ptr).to_string_lossy().to_string();

    unescape::unescape(string.as_str()).unwrap()
}

/// Given a function definition node, return the function name and list of parameter names.
pub fn parse_fn(node: Node) -> (String, Vec<String>) {
    let parse_parameter = |pnode: Node| match pnode.token_type() {
        Some(YYTokenType::LEAF) => pnode.as_string().unwrap(),
        Some(YYTokenType::DECLARE) => pnode.right_node().unwrap().as_string().unwrap(),
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

    parameter_list.retain(|s| !s.is_empty());

    (function_name, parameter_list)
}

pub fn parse_args(param: NodePtr) -> Vec<NodePtr> {
    let mut arg_list = vec![];
    let mut param = param;

    log::debug!("(parser) start args parse at {:?}", param);

    while !(param.is_null()) {
        log::trace!("(parser) parse arg at {:?}", param);

        let node = unsafe { param.as_mut().unwrap() };

        if let Some(YYTokenType::Comma) = node.token_type() {
            arg_list.insert(0, node.right);
            param = node.left;
        } else {
            arg_list.insert(0, param);
            break;
        };
    }

    log::debug!("(parser) finish parse args {:?}", arg_list);

    arg_list
}
