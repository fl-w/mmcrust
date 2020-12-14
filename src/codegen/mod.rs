// pub mod optimiser;
pub mod tac;

use log::{debug, trace};
use parser::*;
use std::{collections::HashMap, path::PathBuf};
use Constant::Int;

use self::tac::{Address, Cond, Constant, Tac, TacKind, TacSequence};

#[derive(Default)]
struct Assembly {
    asm: String,
}

impl Into<String> for Assembly {
    fn into(self) -> String { self.asm }
}

impl Assembly {
    fn add<S: Into<String>>(&mut self, string: S) { self.asm.push_str(string.into().as_str()) }

    fn addi<S: Into<String>>(&mut self, string: S) {
        self.asm
            .push_str(format!("{:<4}{:<20}", "", string.into()).as_str())
    }

    fn addl<S: Into<String>>(&mut self, string: S) {
        self.asm.push_str(format!("{}\n", string.into()).as_str())
    }

    fn add_all<S: Into<String>>(&mut self, strings: Vec<S>) {
        for string in strings {
            self.asm.push_str(string.into().as_str())
        }
    }
}

#[derive(Debug, Default)]
pub struct Generator {
    conditional_count: i32,
    symbol_table: HashMap<char, String>,
}

struct Frame {
    formals: usize,
    offlst: Vec<usize>,
    locals: usize,
    maxargs: Box<usize>,
}

struct Level {
    frame: Frame,
    slink_offset: usize,
    parent: Box<Level>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ActivationRecord {
    fp: Box<ActivationRecord>,
    param: Vec<Address>,
    pc: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SymbolScope {
    Global,
    Local,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Symbol {
    pub scope: SymbolScope,
    pub index: u16,
}
struct SymbolLayer {
    store: HashMap<SymbolName, Symbol>,
    num_of_vars: usize,
}

#[derive(Default)]
struct SymbolTable {
    store: HashMap<CompiledFunction, SymbolLayer>,
}

impl Generator {
    pub fn generate(&mut self, prog: TacSequence) -> String {
        let mut asm = Assembly::default();
        let mut function = None; // store the current function
        let mut sp = 0;
        let mut symbol_table = SymbolTable::default();

        for code in prog {
            asm.add("# ");
            asm.addl(code.to_string());

            match &code.kind {
                TacKind::BranchTarget => asm.addi(format!("{}:", code.target)),

                TacKind::Store(Address::Const(Int(k))) => {
                    asm.addi(format!("li ${}, {}", code.target, k))
                }

                TacKind::Store(ref from) => asm.addi(format!(
                    "add{} {}, $R0, {}",
                    if matches!(from, Address::Const(_)) {
                        "i"
                    } else {
                        ""
                    },
                    code.target,
                    from
                )),

                TacKind::Call(args) => {
                    // move stack pointer to accomadate args
                    asm.addi(format!("subi $sp, $sp {}", 4 * args.len()));

                    let mut sp_disp = 4;
                    for arg in args {
                        asm.addi(format!("li $t0, {}", arg))
                    }
                }

                TacKind::BinOp(operator, Address::Name(a), Address::Name(b)) => {
                    asm.addi(format!("lw $t0", a));
                    asm.addi(format!("lw $t1", b));
                    asm.addi(format!("add {} {}", a, b));
                }

                TacKind::BinOp(BinOp::Add, left, Address::Const(k)) => {
                    asm.addi(format!("addi {}, {}, {}", code.target, left, k))
                }

                TacKind::BinOp(BinOp::Add, left, right) => {
                    asm.addi(format!("add {}, {}, {}", code.target, left, right))
                }

                TacKind::Branch => asm.addi(format!("j {}", code.target)),

                TacKind::CBranch(cond, left, right) => asm.addi(format!("b{:?} {}, {}, {}", cond, 
                    symbol_table.get(left),
                    symbol_table.get(right)

                             code.target)),

                TacKind::BinOp(BinOp::Add, left, right) => asm.addi(format!(
                    "add {}, {}, {}",
                    code.target,
                    symbol_table.get(left),
                    symbol_table.get(right)
                )),

                TacKind::BinOp(BinOp::Sub, left, right) => {
                    asm.addi(format!("sub {}, {}, {}", code.target, left, right))
                }

                _ => {}
            };
        }
        asm.into()
    }
}

#[derive(Default)]
pub struct Program {
    globals: Vec<String>,
    index_map: HashMap<String, isize>,
    num_of_temps: usize,
}

pub fn compile_prog(ast: NodePtr) -> String {
    debug!("starting compile at {:?}", ast);

    let intermediate_code = tac::transpile_ast(ast);

    // let intermediate_code = optimize(intermediate_code);

    debug!(
        "transpiled 3ac\n{}",
        intermediate_code
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n")
    );

    //
    // run code gen

    Generator::default().generate(intermediate_code)
}
