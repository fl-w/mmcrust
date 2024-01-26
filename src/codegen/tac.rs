use log::{debug, trace};
use std::{
    convert::{TryFrom, TryInto},
    fmt, mem,
    os::raw::c_int,
};

use super::symbol_table::SymbolTable;
use parser::{BinOp, FuncDef, Node, NodePtr, YYTokenType};
use Addr::{Const, Name};
use Constant::*;

pub type Program = Vec<Tac>;

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

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Addr {
    Const(Constant),
    Name(String),
    Temporary(usize),
    Label(String, usize),
}

impl fmt::Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Const(c) => c.to_string(),
                Self::Name(v) => v.to_string(),
                Self::Label(prefix, n) => format!(
                    "{}{}",
                    prefix,
                    if *n == 1 {
                        "".to_owned()
                    } else {
                        n.to_string()
                    }
                ),
                Self::Temporary(v) => format!("t{}", v),
            }
        )
    }
}

impl fmt::Debug for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Const(Str(s)) => format!("\"{}\"", s),
                // Self::Name(name) =>
                //     if name != "main" {
                //         format!("_{}", name)
                //     } else {
                //         name.clone()
                //     },
                Self::Label(_, _) => format!("{}:", self),
                Self::Temporary(v) => format!("$t{}", v),
                _ => format!("{}", self),
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
pub struct CompiledFunction {
    pub def: FuncDef,
    pub body: Program,
    pub end: Addr,
    pub num_of_temps: usize,
}

impl CompiledFunction {
    pub fn new(end: Addr, def: FuncDef) -> Self {
        Self {
            body: Vec::new(),
            def,
            end,
            num_of_temps: 0,
        }
    }

    pub fn is_leaf(&self) -> bool {
        !self
            .body
            .iter()
            .any(|tac| matches!(tac.kind, TacKind::Call(_, _)))
    }

    pub fn find_locals(&self) -> Vec<String> {
        self.body
            .iter()
            .map(|tac| {
                tac.ref_addr()
                    .into_iter()
                    .filter_map(|addr| {
                        if let Addr::Name(n) = addr {
                            Some(n.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect::<Vec<String>>()
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum TacKind {
    Store(Addr),

    BinOp(BinOp, Addr, Addr),

    BranchTarget,

    Branch,

    CBranch(Cond, Addr, Addr),

    Function(CompiledFunction),

    Call(Vec<Addr>, Option<Addr>),

    Return(Option<Addr>),

    UnaryOp(BinOp, Addr),

    Nop, // Closure(Address),
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct Tac {
    pub target: Addr,
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

            TacKind::Function(func) => write!(
                f,
                "fn begin {} {} \n {} \nend({0})",
                dst,
                func.def.parameters.len(),
                func.body
                    .iter()
                    .map(|tac| format!("\t{}", tac.to_string().replace("\n", "\n\t")))
                    .collect::<Vec<String>>()
                    .join("\n")
            ),

            TacKind::Return(ref value) => write!(
                f,
                "return {}",
                value
                    .clone()
                    .map(|a| a.to_string())
                    .unwrap_or_else(String::new)
            ),

            TacKind::Call(args, dest) => writeln!(
                f,
                "{}",
                args.iter()
                    .map(|param| format!("param {}", param))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
            .and(write!(
                f,
                "{}call {} {}",
                dest.clone()
                    .map(|d| format!("{} = ", d))
                    .unwrap_or_default(),
                dst,
                args.len()
            )),

            Nop => Ok(()),
        }
    }
}

impl Into<String> for Tac {
    fn into(self) -> String { self.to_string() }
}

impl Tac {
    fn new(dst: Addr, inst: TacKind) -> Self {
        Self {
            target: dst,
            kind: inst,
        }
    }

    pub fn clone_to(&self, dst: Addr) -> Self {
        let mut clone = self.clone();
        clone.target = dst;

        clone
    }
}

impl Tac {
    /// get mutable vector of all referenced addresses
    pub fn ref_addr_mut(&mut self) -> Vec<&mut Addr> {
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
    pub fn ref_addr(&self) -> Vec<&Addr> {
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

    pub fn has_src(&self, addr: &Addr) -> bool {
        println!("self: {:?}, with {:?}", self, addr);
        let dst = &self.target;

        self.ref_addr()
            .into_iter()
            .any(|ref_addr| ref_addr != dst && ref_addr == addr)
    }
}

// struct TacTranspilerScope {}

#[derive(Default)]
struct TacTranspiler {
    stack: Program,
    current_loop: Option<(Addr, Addr)>,
    current_function: Option<CompiledFunction>,
    temp_c: usize,
}

impl TacTranspiler {
    fn new() -> Self { Default::default() }

    fn last_dst(&self) -> Option<&Addr> { self.stack().last().map(|tac| &tac.target) }

    fn stack(&self) -> &Program {
        if let Some(ref func) = self.current_function {
            &func.body
        } else {
            &self.stack
        }
    }

    fn stack_mut(&mut self) -> &mut Program {
        if let Some(ref mut func) = self.current_function {
            &mut func.body
        } else {
            &mut self.stack
        }
    }

    fn temp_c(&mut self) -> &mut usize {
        if let Some(ref mut function) = self.current_function {
            &mut function.num_of_temps
        } else {
            &mut self.temp_c
        }
    }

    fn next_temp(&mut self) -> Addr {
        let temp_counter = self.temp_c();
        let addr = Addr::Temporary(*temp_counter);
        *temp_counter += 1;

        addr
    }

    fn free_temp(&mut self, addr: &Addr) {
        if let Addr::Temporary(_) = addr {
            *self.temp_c() -= 1;
        }
    }

    /// TODO: maybe keep a hashtable of store of counters?
    fn next_jump_target(&self, suffix: &str) -> Addr {
        let n = self
            .stack
            .iter()
            .filter(|code| match code {
                Tac {
                    kind: TacKind::BranchTarget,
                    target: Addr::Label(p, _),
                } => *p == suffix,
                _ => false,
            })
            .count();

        let mut label = if let Some(ref function) = self.current_function {
            format!("{:?}_", Name(function.def.name.clone()))
        } else {
            String::new()
        };

        label.push_str(suffix);

        Addr::Label(label, n + 1)
    }

    fn add_instruction(&mut self, dst: Addr, inst: TacKind) -> &Addr {
        let code = Tac::new(dst, inst);

        debug!("(transpiler) add_inst {}", code);

        self.stack_mut().push(code);
        self.last_dst().unwrap()
    }

    fn add_copy(&mut self, src: Addr, dst: Addr) -> Addr {
        // if (dst.)
        self.stack_mut()
            .push(Tac::new(dst.clone(), TacKind::Store(src)));

        dst
    }

    fn add_assignment(&mut self, node: Node) -> Addr {
        let name = node.left_node().and_then(|node| node.as_string()).unwrap();
        let expr = self.walk(node.right_node().unwrap());

        let name = Name(name);

        if let Some(Tac {
            target,
            kind: TacKind::Call(a, None),
        }) = self.stack().last()
        {
            if &expr == target {
                self.stack_mut().last_mut().unwrap().kind = TacKind::Call(a.clone(), Some(name));
                return expr;
            }
        }

        self.add_copy(expr, name)
    }

    fn jump(&mut self, jump_target: &Addr) -> &Addr {
        self.add_instruction(jump_target.clone(), TacKind::Branch)
    }

    fn add_jump_target(&mut self, jump_target: &Addr) -> &Addr {
        self.add_instruction(jump_target.clone(), TacKind::BranchTarget)
    }

    fn add_condition(&mut self, node: Node) -> (Cond, Addr, Addr) {
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

    fn add_if_block(&mut self, node: Node) -> Addr {
        let jump_target = self.next_jump_target("if_exit");
        let (test, left, right) = self.add_condition(node.left_node().unwrap());

        self.add_instruction(
            jump_target.clone(),
            // invert test here for optimal branching
            TacKind::CBranch(test.invert(), left, right),
        );

        if let Some((node, Some(token))) = Node::deref_node_and_token(node.right) {
            if let YYTokenType::ELSE = token {
                let else_jump_target = self.next_jump_target("else_exit");
                self.walk_tree(node.left);
                self.jump(&else_jump_target);
                self.add_jump_target(&jump_target);

                self.walk(node.right_node().unwrap());

                self.add_jump_target(&else_jump_target);

                else_jump_target
            } else {
                self.walk_tree(node.ptr());
                self.add_jump_target(&jump_target);

                jump_target
            }
        } else {
            panic!("parse error: could not parse if block")
        }
    }

    fn add_while_block(&mut self, node: Node) -> Addr {
        let (test, left, right) = self.add_condition(node.left_node().unwrap());

        let loop_start = self.next_jump_target("while");
        self.add_jump_target(&loop_start);

        let loop_end = self.next_jump_target("while_exit");
        self.add_instruction(
            loop_end.clone(),
            // invert test here for optimal branching
            TacKind::CBranch(test.invert(), left, right),
        );

        self.current_loop = Some((loop_start.clone(), loop_end.clone()));

        if let Some(right_node) = node.right_node() {
            self.walk(right_node);
            self.jump(&loop_start);
        }

        self.add_jump_target(&loop_end);

        loop_end
    }

    fn add_fn(&mut self, node: Node) -> Addr {
        // parser shouldn't allow any other case. but
        // if let here to be safe
        if let Some((def, YYTokenType::d)) = node.left_node_and_token() {
            let return_type = def.left_node().and_then(Node::as_string).unwrap();
            let (name, parameters) = parser::parse_fn(def.right_node().unwrap());

            log::debug!("add_fn {} {:?}", name, parameters);

            let name_addr = Addr::Name(name.clone());
            let function = CompiledFunction::new(
                Addr::Temporary(0),
                FuncDef {
                    parameters,
                    return_type,
                    name,
                },
            );

            let outer_function = self.current_function.replace(function);
            let end = self.next_jump_target("return");
            if let Some(ref mut f) = self.current_function {
                f.end = end;
            }

            self.walk_tree(node.right); // walk function body

            let function = mem::replace(&mut self.current_function, outer_function);

            self.add_instruction(name_addr.clone(), TacKind::Function(function.unwrap()));
            name_addr
        } else {
            panic!("parse error")
        }
    }
    pub fn last_inst_is_call(&self) -> bool {
        matches!(
            self.stack.last(),
            Some(Tac {
                target: _,
                kind: TacKind::Call(_, None),
            })
        )
    }

    pub fn add_call(&mut self, node: Node) -> Addr {
        let name = self.walk(node.left_node().unwrap());
        let args: Vec<Addr> = parser::parse_args(node.right)
            .into_iter()
            .map(TryInto::try_into)
            .filter_map(Result::ok)
            .map(|arg: Node| {
                let mut addr = self.walk(arg);
                self.free_temp(&addr);

                if self.last_inst_is_call() {
                    addr = self.next_temp();
                    let tac = self.stack_mut().last_mut().unwrap();
                    if let TacKind::Call(_, ref mut dest) = &mut tac.kind {
                        dest.replace(addr.clone());
                    };
                }

                addr
            })
            .collect();

        trace!(
            "walking call {}({})",
            name,
            args.iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        );

        self.add_instruction(name, TacKind::Call(args, None))
            .clone()
    }

    fn add_return(&mut self, node: Node) -> Addr {
        let value = if let Some(left_node) = node.left_node() {
            Some(self.walk(left_node))
        } else {
            None
        };

        self.add_instruction(Const(Int32(0)), TacKind::Return(value))
            .clone()
    }

    fn add_decl(&mut self, node: Node) -> Addr {
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

    fn add_expr(&mut self, node: Node, op: BinOp) -> Addr {
        let left_node = node.left_node().unwrap();
        let src_left = self.walk(left_node);

        let inst = if let Some(right_node) = node.right_node() {
            let src_right = self.walk(right_node);
            self.free_temp(&src_left);
            self.free_temp(&src_right);

            TacKind::BinOp(op, src_left, src_right)
        } else {
            self.free_temp(&src_left);
            TacKind::UnaryOp(op, src_left)
        };

        let dst = self.next_temp();
        self.add_instruction(dst, inst).clone()
    }

    fn add_loop_jump(&mut self, break_or_continue: &str) -> Addr {
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
    fn walk(&mut self, node: Node) -> Addr {
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
                YYTokenType::StringLiteral => Const(Str(node.as_string().unwrap())),
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

pub fn transpile_ast(ast_root: NodePtr) -> Program {
    let mut transpiler = TacTranspiler::new();

    transpiler.walk_tree(ast_root);

    transpiler.stack
}
