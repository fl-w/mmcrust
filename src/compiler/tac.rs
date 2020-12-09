use log::trace;
use std::{
    convert::{TryFrom, TryInto},
    fmt,
    os::raw::c_int,
};

use parser::{Infix, Node, NodePtr, YYTokenType};
use Address::{Const, Name};

pub type TacSequence = Vec<ThreeAddressCode>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Address {
    Const(c_int),
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
                Self::Label(prefix, n) => format!("{}{}", prefix, n),
                Self::Temporary(v) => format!("t{}", v),
            }
        )
    }
}

// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
// pub enum Condition {
//     Bool(bool),
//     Relational(Infix, Address, Address),
// }

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Cond {
    EQ,
    LT,
    GT,
    LE,
    GE,
    NE,
}

impl TryFrom<Infix> for Cond {
    type Error = ();

    fn try_from(infix: Infix) -> Result<Self, Self::Error> {
        match infix {
            Infix::LessEqual => Ok(Self::LE),
            Infix::Greater => Ok(Self::GT),
            Infix::Equal => Ok(Self::EQ),
            Infix::GreaterEqual => Ok(Self::GE),
            Infix::Less => Ok(Self::LT),
            Infix::NotEqual => Ok(Self::NE),
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
                Self::LE => Infix::LessEqual,
                Self::GT => Infix::Greater,
                Self::EQ => Infix::Equal,
                Self::GE => Infix::GreaterEqual,
                Self::LT => Infix::Less,
                Self::NE => Infix::NotEqual,
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
pub enum Inst {
    Copy(Address),

    UnaryOp(Infix, Address),

    BinOp(Infix, Address, Address),

    BranchTarget,

    Branch,

    CBranch(Cond, Address, Address),

    ProcBegin(usize),

    ProcEnd,

    Call(Vec<Address>), // EndProc,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ActivationRecord {
    fp: Box<ActivationRecord>,
    param: Vec<Address>,
    pc: usize,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct ThreeAddressCode {
    pub dst: Address,
    pub inst: Inst,
}

impl fmt::Debug for ThreeAddressCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> { write!(f, "{}", self) }
}

impl fmt::Display for ThreeAddressCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let dst = &self.dst;
        match &self.inst {
            Inst::BranchTarget => write!(f, "label {}", dst),

            Inst::Branch => write!(f, "goto {}", dst),

            Inst::Copy(src) => write!(f, "{} = {}", dst, src),

            Inst::BinOp(op, left, right) => {
                write!(f, "{} = {} {} {}", dst, left, op, right)
            }

            Inst::UnaryOp(op, src) => {
                write!(f, "{} = {} {}", dst, op, src)
            }

            Inst::CBranch(cond, left, right) => {
                write!(f, "if {}{}{} goto {} ", left, cond, right, dst)
            }

            Inst::ProcBegin(num_param) => write!(f, "func begin {} {}", dst, num_param),

            Inst::ProcEnd => write!(f, "func end"),

            Inst::Call(args) => write!(f, "{}()", dst),
        }
    }
}

impl Into<String> for ThreeAddressCode {
    fn into(self) -> String { self.to_string() }
}

impl ThreeAddressCode {
    fn new(dst: Address, inst: Inst) -> Self { Self { dst, inst } }

    pub fn clone_to(&self, dst: Address) -> Self {
        let mut clone = self.clone();
        clone.dst = dst;

        clone
    }
}

impl ThreeAddressCode {
    /// get mutable vector of all referenced addresses
    pub fn ref_addr_mut(&mut self) -> Vec<&mut Address> {
        let mut ref_addr = vec![&mut self.dst];

        match self.inst {
            Inst::UnaryOp(_, ref mut src) | Inst::Copy(ref mut src) => ref_addr.push(src),
            Inst::BinOp(_, ref mut left, ref mut right) => {
                ref_addr.push(left);
                ref_addr.push(right);
            }
            _ => (),
        }

        ref_addr
    }

    /// get immutable vector of all referenced addresses
    pub fn ref_addr(&self) -> Vec<&Address> {
        let mut ref_addr = vec![&self.dst];

        match self.inst {
            Inst::UnaryOp(_, ref src) | Inst::Copy(ref src) => ref_addr.push(src),
            Inst::BinOp(_, ref left, ref right) => {
                ref_addr.push(left);
                ref_addr.push(right);
            }
            _ => (),
        }

        ref_addr
    }

    pub fn has_src(&self, addr: &Address) -> bool {
        println!("self: {:?}, with {:?}", self, addr);
        let dst = &self.dst;

        self.ref_addr()
            .into_iter()
            .any(|ref_addr| ref_addr != dst && ref_addr == addr)
    }
}

#[derive(Default)]
struct TacTranspiler {
    stack: TacSequence,
    temp_counter: usize,
}

impl TacTranspiler {
    fn new() -> Self { Self::default() }

    fn last_dst(&self) -> Option<&Address> { self.stack.last().map(|tac| &tac.dst) }

    fn next_temp_addr(&mut self) -> Address {
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
                ThreeAddressCode {
                    inst: Inst::BranchTarget,
                    dst: Address::Label(p, _),
                } => *p == prefix,
                _ => false,
            })
            .count();

        Address::Label(prefix, n + 1)
    }

    fn add_instruction(&mut self, dst: Address, inst: Inst) -> &Address {
        self.stack.push(ThreeAddressCode::new(dst, inst));

        self.last_dst().unwrap()
    }

    fn add_copy(&mut self, src: Address, dst: Address) {
        self.stack.push(ThreeAddressCode::new(dst, Inst::Copy(src)));
    }

    fn add_expr(&mut self, node: Node) -> Address {
        match node.token_type() {
            Some(YYTokenType::Infix(op)) => {
                let left_node = node.left_node().unwrap();
                let src_left = self.add_expr(left_node);

                let inst = if let Some(right_node) = node.right_node() {
                    let src_right = self.add_expr(right_node);

                    Inst::BinOp(op, src_left, src_right)
                } else {
                    Inst::UnaryOp(op, src_left)
                };

                let dst = self.next_temp_addr();
                self.add_instruction(dst, inst).clone()
            }
            Some(YYTokenType::LEAF) => self.add_expr(node.left_node().unwrap()),
            Some(YYTokenType::CONSTANT) => Const(node.as_cint().unwrap()),
            Some(YYTokenType::IDENTIFIER) => Name(node.as_string().unwrap()),
            _ => unreachable!("add_expr: {:?}", node.token_type()),
        }
    }

    fn add_assignment(&mut self, node: Node) {
        let name = node.left_node().unwrap().as_string().unwrap();
        let addr = self.add_expr(node.right_node().unwrap());

        self.add_copy(addr, Name(name));
    }

    fn add_jump(&mut self, jump_target: Address) -> &Address {
        self.add_instruction(jump_target, Inst::Branch)
    }

    fn add_jump_target(&mut self, jump_target: Address) -> &Address {
        self.add_instruction(jump_target, Inst::BranchTarget)
    }

    fn add_condition(&mut self, node: Node) -> (Cond, Address, Address) {
        match node.token_type() {
            Some(YYTokenType::Infix(op)) => {
                let left = self.add_expr(node.left_node().unwrap());
                let right = self.add_expr(node.right_node().unwrap());
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

    fn add_conditional_block(&mut self, node: Node) {
        let jump_target = self.next_jump_target("L");
        let (test, left, right) = self.add_condition(node.left_node().unwrap());

        self.add_instruction(
            jump_target.clone(),
            Inst::CBranch(test.invert(), left, right),
        );

        if let Some((node, Some(token))) = Node::deref_node_and_token(node.right) {
            // if (condition) { consequence }
            //
            // should emit:
            //
            // 1. condition
            // 2. jump to 5 if not truthy
            // 3. consequence (without the last pop)
            // 4. jump to 6
            // 5. push null
            // 6. pop
            //
            // --
            //
            // if (condition) { consequence } else { alternative }
            //
            // should emit:
            //
            // 1. condition
            // 2. jump to 5 if not truthy
            // 3. consequence (without the last pop)
            // 4. jump to 6
            // 5. alternative (without the last pop)
            // 6. pop
            self.walk_tree(node.left);
            if let YYTokenType::ELSE = token {
                let else_jump_target = self.next_jump_target("L");

                self.add_jump(else_jump_target.clone());
                self.add_jump_target(jump_target);

                self.walk_tree(node.right);
                self.add_jump_target(else_jump_target);
            } else {
                self.add_jump_target(jump_target);
            }
        }
    }

    pub fn add_func(&mut self, node: Node) {
        // parser shouldn't allow any other case. but
        // if let here to be safe
        if let Some((def, YYTokenType::d)) = node.left_node_and_token() {
            // let return_type = def.left_node().and_then(Node::as_string).unwrap();
            let (name, parameters) = parser::parse_fn(def.right_node().unwrap());

            log::debug!("{} {:?}", name, parameters);

            let name_addr = Address::Name(name);

            self.add_instruction(name_addr, Inst::ProcBegin(parameters.len()));
            self.walk_tree(node.right);
        }
    }

    pub fn add_call(&mut self, node: Node) -> Address {
        let func = self.add_expr(node.left_node().unwrap());
        let args: Vec<Address> = parser::parse_args(node.right)
            .into_iter()
            .map(TryInto::try_into)
            .filter_map(Result::ok)
            .map(|arg: Node| self.add_expr(arg))
            .collect();

        trace!(
            "walking call {}({})",
            func,
            args.iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        );
    }

    pub fn walk_tree(&mut self, ptr: NodePtr) {
        if let Some((node, Some(token))) = Node::deref_node_and_token(ptr) {
            trace!("walking tree at {:?} with token {:?}", ptr, token);
            match token {
                YYTokenType::Assign => self.add_assignment(node),
                YYTokenType::IF => self.add_conditional_block(node),
                // D: left is the function info (d), right is the function
                YYTokenType::D => self.add_func(node),
                YYTokenType::APPLY => self.add_call(node),
                YYTokenType::Declaration => self.walk_tree(node.right),
                YYTokenType::Infix(_) => {
                    self.add_expr(node);
                }
                YYTokenType::LEAF => (),
                _ => {
                    self.walk_tree(node.left);
                    self.walk_tree(node.right)
                }
            }
        }
    }
    // result.push_str(format!("\n    .globl    {}\n    .type {}, @function\n\n", fnc.name.clone(), fnc.name.clone()).as_str());

    pub fn walk_function(&mut self, ptr: NodePtr) {}
}

pub struct CompiledFunction {}

// struct IntermediateCode {
//     globals
// }

pub fn transpile_ast(ast_root: NodePtr) -> TacSequence {
    let mut transpiler = TacTranspiler::new();

    transpiler.walk_tree(ast_root);

    transpiler.stack
}
