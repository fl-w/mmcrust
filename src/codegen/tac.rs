use log::{debug, trace};
use std::{
    convert::{TryFrom, TryInto},
    fmt,
    os::raw::c_int,
};

use parser::{BinOp, FnDef, Node, NodePtr, YYTokenType};
use Address::{Const, Name};
use Constant::*;

use super::Program;

pub type TacSequence = Vec<Tac>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Constant {
    Int32(c_int),
    Str(String),
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Int32(i) => write!(f, "{}", i),
            Self::Str(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Address {
    Const(Constant),
    Name(String),
    Temporary(usize),
    Label(&'static str, usize),
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Const(int) => int.to_string(),
                Self::Name(v) => v.to_string(),
                // Self::String(v) => format!("\"{}\"", v.to_string(),),
                Self::Label(prefix, n) => format!("{}{}", prefix, n),
                Self::Temporary(v) => format!("t{}", v),
            }
        )
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Cond {
    EQ,
    LT,
    GT,
    LE,
    GE,
    NE,
}

impl TryFrom<BinOp> for Cond {
    type Error = ();

    fn try_from(infix: BinOp) -> Result<Self, Self::Error> {
        match infix {
            BinOp::LessEqual => Ok(Self::LE),
            BinOp::Greater => Ok(Self::GT),
            BinOp::Equal => Ok(Self::EQ),
            BinOp::GreaterEqual => Ok(Self::GE),
            BinOp::Less => Ok(Self::LT),
            BinOp::NotEqual => Ok(Self::NE),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Cond {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::LE => BinOp::LessEqual,
                Self::GT => BinOp::Greater,
                Self::EQ => BinOp::Equal,
                Self::GE => BinOp::GreaterEqual,
                Self::LT => BinOp::Less,
                Self::NE => BinOp::NotEqual,
            }
        )
    }
}

impl fmt::Debug for Cond {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Cond::LT => "lt",
                Cond::GT => "gt",
                Cond::LE => "le",
                Cond::GE => "ge",
                Cond::EQ => "eq",
                Cond::NE => "ne",
            }
        )
    }
}

impl Cond {
    fn invert(self) -> Self {
        match self {
            Self::LT => Self::GE,
            Self::GT => Self::LE,
            Self::LE => Self::GT,
            Self::GE => Self::LT,
            Self::EQ => Self::NE,
            Self::NE => Self::EQ,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum TacKind {
    Store(Address),

    BinOp(BinOp, Address, Address),

    BranchTarget,

    Branch,

    CBranch(Cond, Address, Address),

    ProcBegin(usize),

    ProcEnd,

    Call(Vec<Address>),

    Return(Option<Address>),

    UnaryOp(BinOp, Address),

    Nop, // Closure(Address),
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Tac {
    pub target: Address,
    pub kind: TacKind,
}

impl fmt::Debug for Tac {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> { write!(f, "{}", self) }
}

impl fmt::Display for Tac {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let dst = &self.target;
        match &self.kind {
            TacKind::BranchTarget => write!(f, "label {}", dst),

            TacKind::Branch => write!(f, "goto {}", dst),

            TacKind::Store(src) => write!(f, "{} = {}", dst, src),

            TacKind::BinOp(op, left, right) => {
                write!(f, "{} = {} {} {}", dst, left, op, right)
            }

            TacKind::UnaryOp(op, src) => {
                write!(f, "{} = {} {}", dst, op, src)
            }
            TacKind::CBranch(cond, left, right) => {
                write!(f, "if {}{}{} goto {} ", left, cond, right, dst)
            }

            TacKind::ProcBegin(num_param) => write!(f, "fn begin {} {}", dst, num_param),

            TacKind::ProcEnd => write!(f, "end({})", dst),

            TacKind::Return(ref value) => write!(
                f,
                "return {}",
                value
                    .clone()
                    .map(|a| a.to_string())
                    .unwrap_or_else(String::new)
            ),

            TacKind::Call(args) => write!(
                f,
                "{}",
                args.iter()
                    .map(|param| format!("param {}", param))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
            .and(write!(f, "call {} {}", dst, args.len())),

            Nop => Ok(()),
        }
    }
}

impl Into<String> for Tac {
    fn into(self) -> String { self.to_string() }
}

impl Tac {
    fn new(dst: Address, inst: TacKind) -> Self {
        Self {
            target: dst,
            kind: inst,
        }
    }

    pub fn clone_to(&self, dst: Address) -> Self {
        let mut clone = self.clone();
        clone.target = dst;

        clone
    }
}

impl Tac {
    /// get mutable vector of all referenced addresses
    pub fn ref_addr_mut(&mut self) -> Vec<&mut Address> {
        let mut ref_addr = vec![&mut self.target];

        match self.kind {
            TacKind::UnaryOp(_, ref mut src) | TacKind::Store(ref mut src) => ref_addr.push(src),
            TacKind::BinOp(_, ref mut left, ref mut right) => {
                ref_addr.push(left);
                ref_addr.push(right);
            }
            _ => (),
        }

        ref_addr
    }

    /// get immutable vector of all referenced addresses
    pub fn ref_addr(&self) -> Vec<&Address> {
        let mut ref_addr = vec![&self.target];

        match self.kind {
            TacKind::UnaryOp(_, ref src) | TacKind::Store(ref src) => ref_addr.push(src),
            TacKind::BinOp(_, ref left, ref right) => {
                ref_addr.push(left);
                ref_addr.push(right);
            }
            _ => (),
        }

        ref_addr
    }

    pub fn has_src(&self, addr: &Address) -> bool {
        println!("self: {:?}, with {:?}", self, addr);
        let dst = &self.target;

        self.ref_addr()
            .into_iter()
            .any(|ref_addr| ref_addr != dst && ref_addr == addr)
    }
}

#[derive(Default)]
struct TacTranspiler {
    stack: TacSequence,
    temp_counter: usize,
    current_loop: Option<(Address, Address)>,
    current_function: Option<FnDef>,
    globals: Vec<String>,
}

impl TacTranspiler {
    fn new() -> Self { Self::default() }

    fn last_dst(&self) -> Option<&Address> { self.stack.last().map(|tac| &tac.target) }

    fn next_temp(&mut self) -> Address {
        let addr = Address::Temporary(self.temp_counter);
        self.temp_counter += 1;

        addr
    }

    /// TODO: maybe keep a hashtable of store of counters?
    fn next_jump_target(&self, prefix: &'static str) -> Address {
        let n = self
            .stack
            .iter()
            .filter(|code| match code {
                Tac {
                    kind: TacKind::BranchTarget,
                    target: Address::Label(p, _),
                } => *p == prefix,
                _ => false,
            })
            .count();

        Address::Label(prefix, n + 1)
    }

    fn add_instruction(&mut self, dst: Address, inst: TacKind) -> &Address {
        let code = Tac::new(dst, inst);

        debug!("(transpiler) add_inst {}", code);

        self.stack.push(code);

        self.last_dst().unwrap()
    }

    fn add_copy(&mut self, src: Address, dst: Address) -> Address {
        self.stack.push(Tac::new(dst.clone(), TacKind::Store(src)));

        dst
    }

    fn add_assignment(&mut self, node: Node) -> Address {
        let name = node.left_node().and_then(|node| node.as_string()).unwrap();
        let expr = self.walk(node.right_node().unwrap());

        self.add_copy(expr, Name(name))
    }

    fn jump(&mut self, jump_target: &Address) -> &Address {
        self.add_instruction(jump_target.clone(), TacKind::Branch)
    }

    fn add_jump_target(&mut self, jump_target: &Address) -> &Address {
        self.add_instruction(jump_target.clone(), TacKind::BranchTarget)
    }

    fn add_condition(&mut self, node: Node) -> (Cond, Address, Address) {
        match node.token_type() {
            Some(YYTokenType::Infix(op)) => {
                let left = self.walk(node.left_node().unwrap());
                let right = self.walk(node.right_node().unwrap());
                let cond = op.try_into();

                (
                    cond.expect("parse error: could not compile condition test"),
                    left,
                    right,
                )
            }
            _ => panic!("parse error: could not compile condition"),
        }
    }

    fn add_if_block(&mut self, node: Node) -> Address {
        let jump_target = self.next_jump_target("L");
        let (test, left, right) = self.add_condition(node.left_node().unwrap());

        self.add_instruction(
            jump_target.clone(),
            // invert test here for optimal branching
            TacKind::CBranch(test.invert(), left, right),
        );

        if let Some((node, Some(token))) = Node::deref_node_and_token(node.right) {
            self.walk_tree(node.left);

            if let YYTokenType::ELSE = token {
                let else_jump_target = self.next_jump_target("L");

                self.jump(&else_jump_target);
                self.add_jump_target(&jump_target);

                self.walk_tree(node.right);

                self.add_jump_target(&else_jump_target);

                else_jump_target
            } else {
                self.add_jump_target(&jump_target);

                jump_target
            }
        } else {
            panic!("parse error: could not parse if block")
        }
    }

    fn add_while_block(&mut self, node: Node) -> Address {
        let (test, left, right) = self.add_condition(node.left_node().unwrap());

        let loop_start = self.next_jump_target("BWHIILE");
        self.add_jump_target(&loop_start);

        let loop_end = self.next_jump_target("AWHILE");
        self.add_instruction(
            loop_end.clone(),
            // invert test here for optimal branching
            TacKind::CBranch(test.invert(), left, right),
        );

        self.current_loop = Some((loop_start.clone(), loop_end.clone()));

        self.walk(node.right_node().unwrap());
        self.jump(&loop_start);

        self.add_jump_target(&loop_end);

        loop_end
    }

    fn add_fn(&mut self, node: Node) -> Address {
        // parser shouldn't allow any other case. but
        // if let here to be safe
        if let Some((def, YYTokenType::d)) = node.left_node_and_token() {
            // let return_type = def.left_node().and_then(Node::as_string).unwrap();
            let (name, parameters) = parser::parse_fn(def.right_node().unwrap());

            log::debug!("{} {:?}", name, parameters);

            let name_addr = Address::Name(name);

            self.add_instruction(name_addr.clone(), TacKind::ProcBegin(parameters.len()));

            self.walk_tree(node.right); // walk function body

            self.add_instruction(name_addr.clone(), TacKind::ProcEnd);

            name_addr
        } else {
            panic!("parse error")
        }
    }

    pub fn add_call(&mut self, node: Node) -> Address {
        let name = self.walk(node.left_node().unwrap());
        let args: Vec<Address> = parser::parse_args(node.right)
            .into_iter()
            .map(TryInto::try_into)
            .filter_map(Result::ok)
            .map(|arg: Node| self.walk(arg))
            .collect();

        trace!(
            "walking call {}({})",
            name,
            args.iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        );

        self.add_instruction(name, TacKind::Call(args)).clone()
    }

    fn add_return(&mut self, node: Node) -> Address {
        let value = if let Some(left_node) = node.left_node() {
            Some(self.walk(left_node))
        } else {
            None
        };

        self.add_instruction(Const(Int32(0)), TacKind::Return(value))
            .clone()
    }

    fn add_decl(&mut self, node: Node) -> Address {
        let addr = self.walk(node.left_node().unwrap());

        // let is_global = self.current_function.is_none();
        // if is_global {
        //     if let Name(ref value) = addr {
        //         self.globals.push(value.clone());
        //     }
        // }

        if let Some(node) = node.right_node() {
            self.walk(node)
        } else {
            addr
        }
    }

    fn add_expr(&mut self, node: Node, op: BinOp) -> Address {
        let left_node = node.left_node().unwrap();
        let src_left = self.walk(left_node);

        let inst = if let Some(right_node) = node.right_node() {
            let src_right = self.walk(right_node);

            TacKind::BinOp(op, src_left, src_right)
        } else {
            TacKind::UnaryOp(op, src_left)
        };

        let dst = self.next_temp();
        self.add_instruction(dst, inst).clone()
    }

    fn add_loop_jump(&mut self, break_or_continue: &str) -> Address {
        let (start, end) = match &self.current_loop {
            Some((start, end)) => (start.clone(), end.clone()),
            _ => panic!("compiler error: break statement out of loop"),
        };

        let jump = if "break" == break_or_continue {
            end
        } else {
            start
        };

        self.jump(&jump);
        jump
    }

    /// recursive bottom up parsing of the tree
    /// when a node is reached,
    /// - generate labels/address needed
    /// - generate tac code for that node
    /// - add tac to growing list.
    fn walk(&mut self, node: Node) -> Address {
        if let Some(token) = node.token_type() {
            trace!(
                "walking at {:?} with token {:?} ({})",
                &node as *const _,
                token,
                node.type_
            );

            match token {
                YYTokenType::INT => Name("int".to_owned()),
                YYTokenType::CONSTANT => Const(Int32(node.as_cint().unwrap())),
                YYTokenType::STRING_LITERAL => Const(Str(node.as_string().unwrap())),
                YYTokenType::IDENTIFIER => Name(node.as_string().unwrap()),
                YYTokenType::BREAK => self.add_loop_jump("break"),
                YYTokenType::CONTINUE => self.add_loop_jump("continue"),
                YYTokenType::LEAF => self.walk(node.left_node().unwrap()),
                YYTokenType::IF => self.add_if_block(node),
                YYTokenType::WHILE => self.add_while_block(node),
                YYTokenType::ASSIGN => self.add_assignment(node),
                YYTokenType::DECLARE => self.add_decl(node),
                YYTokenType::RETURN => self.add_return(node),
                YYTokenType::Infix(op) => self.add_expr(node, op),
                YYTokenType::D => self.add_fn(node),
                YYTokenType::APPLY => self.add_call(node),
                _ => {
                    let addr = self.walk(node.left_node().unwrap());

                    if let Some(node) = node.right_node() {
                        self.walk(node)
                    } else {
                        addr
                    }
                }
            }
        } else {
            panic!("parse error: invalid token {:?}", node);
        }
    }

    pub fn walk_tree(&mut self, ptr: NodePtr) {
        if let Some((node, Some(_))) = Node::deref_node_and_token(ptr) {
            self.walk(node);
        }
    }
    // result.push_str(format!("\n    .globl    {}\n    .type {}, @function\n\n", fnc.name.clone(), fnc.name.clone()).as_str());
}

pub fn transpile_ast(ast_root: NodePtr) -> TacSequence {
    let mut transpiler = TacTranspiler::new();

    transpiler.walk_tree(ast_root);

    transpiler.stack
}
