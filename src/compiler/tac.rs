use log::trace;
use std::{fmt, os::raw::c_int};

use parser::{Infix, Node, NodePtr, YYTokenType};
use Address::{Const, Name};

pub type IntermediateCode = Vec<ThreeAddressCode>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Address {
    Const(c_int),
    Name(String),
    Temporary(usize),
    Label(String),
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Const(int) => int.to_string(),
                Self::Label(v) | Self::Name(v) => v.to_string(),
                Self::Temporary(v) => format!("t{}", v),
            }
        )
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Inst {
    Copy(Address),

    UnaryOp(Address, Infix),

    BinOp(Address, Address, Infix),

    Label,
    // Branch(Label),
    // BranchCondition {
    //     condition: Condition,
    //     jump: Label,
    // },
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ThreeAddressCode {
    dst: Address,
    instruction: Inst,
}

impl ThreeAddressCode {
    fn new(dst: Address, inst: Inst) -> Self {
        Self {
            dst,
            instruction: inst,
        }
    }

    pub fn clone_to(&self, dst: Address) -> Self {
        let mut clone = self.clone();
        clone.dst = dst;

        clone
    }
}

// #[derive(Debug, Clone, Hash, Eq, PartialEq)]
// pub enum Condition {
//     Bool(bool),
//     Relational { x: Address, y: Address, op: Infix },
// }

impl ThreeAddressCode {
    /// get mutable vector of all referenced addresses
    pub fn ref_addr_mut(&mut self) -> Vec<&mut Address> {
        let ref_addr = vec![&mut self.dst];

        match self.instruction {
            Inst::UnaryOp(ref mut src, _) | Inst::Copy(ref mut src) => ref_addr.push(src),
            Inst::BinOp(ref mut src1, ref mut src2, _) => {
                ref_addr.push(src1);
                ref_addr.push(src2);
            }
            _ => (),
        }

        ref_addr
    }

    /// get immutable vector of all referenced addresses
    pub fn ref_addr(&self) -> Vec<&Address> {
        let ref_addr = vec![&self.dst];

        match self.instruction {
            UnaryOp(ref src, _) | Copy(ref src) => ref_addr.push(src),
            BinOp(ref src1, ref src2, _) => {
                ref_addr.push(src1);
                ref_addr.push(src2);
            }
            _ => (),
        }

        ref_addr
    }

    pub fn has_src(&self, addr: &Address) -> bool {
        let dst = &self.dst;

        self.ref_addr()
            .into_iter()
            .find(|ref_addr| *ref_addr != dst && *ref_addr == addr)
            .is_some()
    }
}

impl fmt::Display for ThreeAddressCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let dst = &self.dst;
        match self.instruction {
            Inst::BinOp(src1, src2, op) => {
                write!(f, "\t{} = {} {} {}", dst, src1, op, src2)
            }
            Inst::UnaryOp(src1, op) => {
                write!(f, "\t{} = {} {}", dst, op, src1)
            }
            Inst::Copy(src) => write!(f, "\t{} = {}", dst, src),
            Inst::Label => write!(f, "L{}:", dst),
        }
    }
}

#[derive(Default)]
struct TacTranspiler {
    stack: IntermediateCode,
}

impl TacTranspiler {
    fn new() -> Self { Self::default() }

    fn last_dst(&self) -> Option<&Address> { self.stack.last().map(|tac| &tac.dst) }

    fn temp_addr(&self) -> Address { Address::Temporary(self.stack.len()) }

    fn add_instruction(&mut self, dst: Address, inst: Inst) -> &Address {
        self.stack.push(ThreeAddressCode::new(dst, inst));

        self.last_dst().unwrap()
    }

    fn add_copy(&mut self, src: Address, dst: Address) {
        self.stack.push(ThreeAddressCode::new(dst, Inst::Copy(src)));
    }

    fn add_ops(&mut self, node: Node) -> Address {
        match node.token_type() {
            Some(YYTokenType::Infix(op)) => {
                let left_node = node.left_node().unwrap();
                let src_left = self.add_ops(left_node);
                let dst = self.temp_addr();

                let inst = if let Some(right_node) = node.right_node() {
                    let src_right = self.add_ops(right_node);

                    Inst::BinOp(src_left, src_right, op)
                } else {
                    Inst::UnaryOp(src_left, op)
                };

                self.add_instruction(dst, inst).clone()
            }
            Some(YYTokenType::LEAF) => self.add_ops(node.left_node().unwrap()),
            Some(YYTokenType::CONSTANT) => Const(node.as_cint().unwrap()),
            Some(YYTokenType::IDENTIFIER) => Name(node.as_string().unwrap()),
            _ => unreachable!(),
        }
    }

    fn add_assignment(&mut self, node: Node) {
        let name = node.left_node().unwrap().as_string().unwrap();
        let addr = self.add_ops(node.right_node().unwrap());

        self.add_copy(addr, Name(name));
    }

    pub fn walk_tree(&mut self, ptr: NodePtr) {
        if let Some((node, Some(token))) = Node::deref_node_and_token(ptr) {
            trace!("walking tree at {:?} with token {:?}", ptr, token);
            match token {
                YYTokenType::Assign => self.add_assignment(node),
                YYTokenType::D => self.walk_tree(node.right),
                YYTokenType::LEAF => (),
                _ => {
                    self.walk_tree(node.left);
                    self.walk_tree(node.right)
                }
            }
        }
    }
}

pub fn transpile_ast(ast_root: NodePtr) -> IntermediateCode {
    let mut transpiler = TacTranspiler::new();

    transpiler.walk_tree(ast_root);

    transpiler.stack
}
