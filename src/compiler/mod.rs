pub mod optimiser;
pub mod tac;

use log::{debug, trace};
use parser::*;
use std::collections::HashMap;

use self::tac::{Address, Cond, Inst, TacSequence, ThreeAddressCode};

#[derive(Default)]
struct Assembly {
    asm: String,
    pub ident_level: usize,
}

impl Into<String> for Assembly {
    fn into(self) -> String { self.asm }
}

impl Assembly {
    fn add<S: Into<String>>(&mut self, string: S) { self.asm.push_str(string.into().as_str()) }

    fn addi<S: Into<String>>(&mut self, string: S) {
        self.asm
            .push_str(format!("{:indent$}{}", string.into(), indent = self.ident_level,).as_str())
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

impl Generator {
    pub fn generate(&mut self, prog: TacSequence) -> String {
        let mut asm = Assembly {
            ident_level: 4,
            ..Default::default()
        };

        for code in prog {
            match &code.inst {
                Inst::BranchTarget => asm.addi(format!("{}:", code.dst)),

                Inst::Copy(ref from) => asm.addi(format!(
                    "add{} {}, $R0, {}",
                    if matches!(from, Address::Const(_)) {
                        "i"
                    } else {
                        ""
                    },
                    code.dst,
                    from
                )),

                Inst::BinOp(Infix::Add, left, Address::Const(k)) => {
                    asm.addi(format!("addi {}, {}, {}", code.dst, left, k))
                }

                Inst::BinOp(Infix::Add, left, right) => {
                    asm.addi(format!("add {}, {}, {}", code.dst, left, right))
                }

                Inst::Branch => asm.addi(format!("j {}", code.dst)),

                Inst::CBranch(cond, left, right) => {
                    asm.addi(format!("b{:?} {}, {}, {}", cond, left, right, code.dst))
                }

                // Inst::ProcBegin()
                _ => {}
            };

            asm.addi("# ");
            asm.addl(code.to_string());
            // asm.addi("\n");
        }
        asm.into()
    }
}

pub fn compile(ast: NodePtr) {
    debug!("starting compile at {:?}", ast);

    let intermediate_code = tac::transpile_ast(ast);

    // let intermediate_code = optimize(intermediate_code);

    debug!(
        "{}",
        intermediate_code
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n")
    );

    //
    // run code gen

    debug!("{}", Generator::default().generate(intermediate_code));
}
