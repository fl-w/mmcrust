// pub mod optimiser;
mod symbol_table;
mod tac;

use log::{debug, trace};
use parser::*;
use std::collections::HashMap;
use Constant::Int32;

use self::tac::{Address, Cond, Constant, Tac, TacKind, TacSequence};
use symbol_table::*;

#[derive(Default, Debug)]
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
    fn addli<S: Into<String>>(&mut self, string: S) {
        self.addi(string);
        self.addl("")
    }

    fn add_all<S: Into<String>>(&mut self, strings: Vec<S>) {
        for string in strings {
            self.asm.push_str(string.into().as_str())
        }
    }

    fn addi_all<S: Into<String>>(&mut self, strings: Vec<S>) {
        for string in strings {
            self.addi(string);
            self.addl("")
        }
    }
}

#[derive(Debug, Default)]
pub struct MipsGenerator {
    conditional_count: i32,
    symbol_table: HashMap<char, String>,
    asm: Assembly,
}

impl MipsGenerator {
    fn emit<S: Into<String>>(&mut self, string: S) { self.asm.addli(string) }

    fn emit_label<S: Into<String>>(&mut self, string: S) { self.asm.addl(string) }

    fn emit_comment<S: Into<String>>(&mut self, string: S) {
        self.asm.add("# ");
        self.asm.addl(string)
    }

    fn gen_printf(&mut self) {
        // Generate printf
        self.asm.addl("_printf:");
        self.asm.addi_all(vec![
            "swc1 $f12, 0($sp)",
            "mtc1 $a0, $f12",
            "li $v0, 2",
            "syscall",
            "lwc1 $f12, 0($sp)",
            "jal $ra",
            "",
        ]);
    }

    /// Generate print_int
    fn gen_print_int(&mut self) {
        self.asm.addl("_print_int:");
        self.asm.addi_all(vec!["li $v0, 1", "syscall", "jal $ra"]);
    }

    /// Generate new line
    fn gen_nl(&mut self) { self.emit_label("_newline:   .asciiz \"\\n\""); }

    fn gen_fnc(&mut self, code: Tac) {
        let func_name = code.target.to_string();

        self.emit_label(format!(".ent {}", func_name));
        self.emit_label(format!("{}:", func_name));
        self.emit(".frame $sp,4,$31");

        // TacKind::BranchTarget => self.assembly.addi(format!("{}:", code.target)),
        self.emit_label(format!(".end {}", func_name));
    }

    pub fn generate(&mut self, prog: TacSequence) {
        self.emit_label(".data");
        self.gen_nl();

        self.emit_label("\n\n.text");
        self.gen_printf();

        for code in prog {
            match &code.kind {
                TacKind::ProcBegin(_) => {
                    self.emit(format!(".globl {}", code.target));
                    // let fn_block = Vec::new();
                    self.gen_fnc(code);
                }

                // TacKind::Store(Address::Const(Int32(k))) => {
                //     assembly.addi(format!("li ${}, {}", code.target, k))
                // }

                // TacKind::Store(ref from) => assembly.addi(format!(
                //     "add{} {}, $R0, {}",
                //     if matches!(from, Address::Const(_)) {
                //         "i"
                //     } else {
                //         ""
                //     },
                //     code.target,
                //     from
                // )),

                // TacKind::Call(args) => {
                //     // move stack pointer to accomadate args
                //     assembly.addi(format!("subi $sp, $sp {}", 4 * args.len()));

                //     let mut sp_disp = 4;
                //     for arg in args {
                //         assembly.addi(format!("li $t0, {}", arg))
                //     }
                // }

                // TacKind::BinOp(operator, Address::Name(a), Address::Name(b)) => {
                //     assembly.addi(format!("lw {}", a));
                //     assembly.addi(format!("lw {}", b));
                //     assembly.addi(format!("add {} {}", a, b));
                // }

                // // add immediate
                // TacKind::BinOp(BinOp::Add, x, Address::Const(Int32(k)))
                // | TacKind::BinOp(BinOp::Add, Address::Const(Int32(k)), x) => {
                //     assembly.addi(format!("addi {}, {}, {}", code.target, x, k))
                // }

                // // add
                // TacKind::BinOp(BinOp::Add, left, right) => {
                //     assembly.addi(format!("add {}, {}, {}", code.target, left, right))
                // }

                // TacKind::Branch => assembly.addi(format!("j {}", code.target)),

                // TacKind::CBranch(cond, left, right) => assembly.addi(format!(
                //     "b{:?} {}, {}, {}",
                //     cond,
                //     symbol_table.get(left),
                //     symbol_table.get(right),
                //     code.target
                // )),

                // TacKind::BinOp(BinOp::Add, left, right) => assembly.addi(format!(
                //     "add {}, {}, {}",
                //     code.target,
                //     symbol_table.get(left),
                //     symbol_table.get(right)
                // )),

                // TacKind::BinOp(BinOp::Sub, left, right) => {
                //     assembly.addi(format!("sub {}, {}, {}", code.target, left, right))
                // }

                // TacKind::Return(value) => {
                //     if let Some(v) = value {
                //         assembly.addi(format!("lw $v0 {}", v));
                //     } else {
                //         // do something
                //     }
                // }
                _ => {}
            };
        }
    }
}

// struct Statement {
//     ir: Tac,
//     function: Func,
//     basic_block: BasicBlock,
// }

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

    let mut mips = MipsGenerator::default();
    mips.generate(intermediate_code);

    mips.asm.into()
}
