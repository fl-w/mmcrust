pub mod optimiser;
mod symbol_table;
mod tac;

use std::collections::HashMap;

use log::debug;
use parser::*;

use self::tac::{Addr, CompiledFunction, Constant, Program, Tac, TacKind};
use optimiser::optimize;
use symbol_table::*;
use Constant::Int32;

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
            .push_str(format!("{:<5}{:<20}", "", string.into()).as_str())
    }

    fn addl<S: Into<String>>(&mut self, string: S) {
        self.asm
            .push_str(format!("{:<20}\n", string.into()).as_str())
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

#[derive(Debug)]
pub enum CompilerError {}

pub struct CompilationScope {
    function: CompiledFunction,
    asm: Assembly,
}

#[derive(Default)]
pub struct MipsCompiler {
    symbol_table: SymbolTable,
    scopes: Vec<CompilationScope>,
    asm: Assembly,
}

impl MipsCompiler {
    fn asm(&mut self) -> &mut Assembly { &mut self.asm }

    fn emit<S: Into<String>>(&mut self, string: S) { self.asm().addli(string) }

    fn emit_label<S: Into<String>>(&mut self, string: S) { self.asm().addl(string) }

    fn emit_with_comment<S: Into<String>>(&mut self, string: S, comment: S) {
        self.asm().addi(string);
        self.asm().add("# ");
        self.asm().addl(comment)
    }

    fn gen_printf(&mut self) {
        // Generate printf
        self.asm().addl("printf:");
        self.asm().addi_all(vec![
            "swc1 $f12, 0($sp)",
            "mtc1 $a0, $f12",
            "li $v0, 2",
            "syscall",
            "lwc1 $f12, 0($sp)",
            "jal $ra",
            "",
        ]);
    }

    /// Generate print_int from a0
    fn gen_print_int(&mut self) {
        self.emit_label("print_int:");
        self.asm().addi_all(vec!["li $v0, 1", "syscall", "jal $ra"]);
    }

    /// Generate print_str from a0
    fn gen_print_str(&mut self) {
        self.emit_label("print_str:");
        self.asm().addi_all(vec!["li $v0, 4", "syscall", "jal $ra"]);
    }
    /// Generate new line
    fn gen_nl(&mut self) { self.emit_label("_newline:   .asciiz \"\\n\""); }

    fn enter_scope(&mut self) {
        // let scope = CompilationScope::default();
        // self.scopes.push(scope);
    }

    fn leave_scope(&mut self) -> Assembly {
        let scope = self.scopes.pop().expect("no scope to leave from");

        scope.asm
    }

    fn compile_fn(
        &mut self,
        func_name: &Addr,
        function: &CompiledFunction,
    ) -> Result<(), CompilerError> {
        let locals = function.find_locals();
        let num_of_locals = locals.len();
        let num_of_params = function.def.parameters.len();
        let func_name = format!("{:?}", func_name);
        let is_main = func_name == "main";
        let is_leaf_function = function.is_leaf();
        let mut registers = HashMap::new();
        let mut offset = 4;

        for (n, arg) in function.def.parameters.iter().enumerate() {
            registers.insert(arg.clone(), format!("$a{}", n));
        }

        let get_reg = move |addr: &Addr| match addr {
            Addr::Name(n) => registers.get(n).cloned().unwrap_or_else(|| {
                let reg = format!("-{}($fp)", 4 * offset);
                registers.insert(n.clone(), reg.clone());
                offset += 4;

                reg
            }),
            // Addr::Temporary(_) => {}
            _ => "".to_owned(),
        };

        self.emit(".text");

        if is_main {
            self.emit(".globl main\n");
        }

        self.emit_label(format!("{}:\t ", func_name));
        self.emit_label(format!(".ent {}", func_name));

        // self.enter_scope();

        // prolouge
        if !is_leaf_function {
            self.emit_with_comment(".frame $sp,4,$31", "begin by reserving space on the stack");
            self.emit_with_comment("sw $ra, -4($sp)", "save return addr");
            self.emit_with_comment("sw $fp, -8($sp)", "save frame pointer");
            self.emit_with_comment("subi $sp, $sp 8", "grow stack");
            self.emit_with_comment("la $fp, 0($sp)", "setup frame pointer");
            self.emit_label("");
        }

        if num_of_locals > 0 {
            self.emit_with_comment(
                format!("sub $sp, $sp, {}", 4 * num_of_locals),
                "add space for locals".to_string(),
            );
            self.emit_label("");
        }

        let mut jumps_to_end = false;
        let mut jump_to_end = false;
        for code in &function.body {
            if jump_to_end {
                self.emit_with_comment(
                    format!("j {}", function.end),
                    "jump to function end".to_string(),
                );
                jump_to_end = false;
                jumps_to_end = true;
            }

            match &code.kind {
                TacKind::Return(ref v) => {
                    if let Some(v) = v {
                        self.emit_with_comment(
                            format!("move $v0 {:?}", v),
                            "move return value to v-reg".to_string(),
                        );
                    }
                    jump_to_end = true
                }

                _ => self.compile_statement(code)?,
            }
        }

        if jumps_to_end {
            self.emit_label("");

            self.emit_label(format!("{:?}", function.end));
        }

        if !is_leaf_function {
            self.emit_label("");

            self.emit_with_comment(
                format!("lw $ra, -{}($fp)", num_of_params * 4),
                "load return address".to_string(),
            );
            self.emit_with_comment("move $t0, $fp", "save value of fp");
            self.emit_with_comment(
                format!("lw $fp, -{}($fp)", 4 + (num_of_params * 4)),
                "restore frame pointer".to_string(),
            );
            self.emit_with_comment("move $sp, $t0", "resore stack pointer");
        }

        self.emit_label("");
        if is_main {
            self.emit_with_comment("li $v0, 10", "perform exit syscall from spim");
            self.emit("syscall");
        } else {
            self.emit_with_comment("jr $ra", "return");
        }

        self.emit_label(format!(".end {}\n", func_name));

        Ok(())
    }

    pub fn load_into_reg(&mut self, reg: String, addr: &Addr) {
        self.emit(format!(
            "{} {}, {:?}",
            if matches!(addr, Addr::Const(Int32(_))) {
                "li"
            } else {
                "lw"
            },
            reg,
            addr
        ));
    }

    pub fn push_into_stack(&mut self, addr: &Addr) {
        self.emit_with_comment(
            format!("sw {:?}, -4($sp)", addr),
            "push into stack".to_string(),
        );
        self.emit_with_comment("la $sp, -4($sp)", "grow stack");
    }

    // pub fn load(&mut self, dest: String, )
    pub fn compile_call(
        &mut self,
        target: &Addr,
        args: &[Addr],
        dest: &Option<Addr>,
    ) -> Result<(), CompilerError> {
        // move stack pointer to accomadate args
        // self.emit(format!("subi $sp, $sp {:?}", 4 * args.len()));

        let (reg_args, stack_args) = if args.len() >= 4 {
            args.split_at(4)
        } else {
            let (a, b) = args.split_at(0);
            (b, a)
        };

        // push first 4 args into a-regs
        for (n, arg) in reg_args.iter().enumerate() {
            self.load_into_reg(format!("$a{}", n), arg)
        }

        for arg in stack_args.iter().rev() {
            self.push_into_stack(arg)
        }

        self.emit(format!("jal {:?}", target));
        self.emit_with_comment(
            "nop",
            "real MIPS architecture is 'pipelined' to improve efficiency",
        );

        if let Some(dest) = dest {
            self.emit(format!("lw {:?}, $v0", dest))
        }

        Ok(())
    }

    pub fn compile_statement(&mut self, code: &Tac) -> Result<(), CompilerError> {
        match &code.kind {
            TacKind::BranchTarget => self.emit_label(format!("{:?}", code.target)),

            TacKind::Store(Addr::Const(Int32(k))) => {
                self.emit_with_comment(format!("li ${:?}, {:?}", code.target, k), code.to_string())
            }

            TacKind::Store(ref from) => self.emit(format!(
                "add{} {:?}, , {:?}",
                if matches!(from, Addr::Const(_)) {
                    "i"
                } else {
                    ""
                },
                code.target,
                from
            )),

            TacKind::Call(args, dest) => self.compile_call(&code.target, args, dest)?,

            // add immediate
            TacKind::BinOp(BinOp::Add, x, Addr::Const(Int32(k)))
            | TacKind::BinOp(BinOp::Add, Addr::Const(Int32(k)), x) => {
                self.emit(format!("addi {:?}, {:?}, {:?}", code.target, x, k))
            }

            // add
            TacKind::BinOp(BinOp::Add, left, right) => {
                self.emit(format!("add {:?}, {:?}, {:?}", code.target, left, right))
            }

            // sub
            TacKind::BinOp(BinOp::Sub, left, right) => {
                self.emit(format!("sub {:?}, {:?}, {:?}", code.target, left, right))
            }

            TacKind::BinOp(operator, left, right) => self.emit(format!(
                "{:?} {:?}, {:?}, {:?}",
                operator, code.target, left, right
            )),

            TacKind::CBranch(cond, left, right) => self.emit(format!(
                "b{:?} {:?}, {:?}, {:?}",
                cond, left, right, code.target
            )),

            TacKind::Return(value) => {
                if let Some(v) = value {
                    self.emit(format!("lw $v0 {:?}", v));
                } else {
                    // do something
                }
            }
            _ => {}
        };

        Ok(())
    }

    pub fn compile(mut self, prog: Program) -> Result<String, CompilerError> {
        self.emit_label(".data\t # variable declarations follow this line");
        self.gen_nl();

        self.emit_label("\n.text\t # instructions follow this line");
        self.gen_print_int();
        self.gen_print_str();

        for statement in &prog {
            match &statement.kind {
                TacKind::Function(ref function) => self.compile_fn(&statement.target, function)?,
                TacKind::Store(ref from) => {
                    self.emit_label(format!(".data\n{:?}:\t.word {}", statement.target, from))
                }
                _ => self.compile_statement(statement)?,
            };
        }

        // Ok(self.scopes[0].asm.into())
        Ok(self.asm.into())
    }
}

// struct Statement {
//     ir: Tac,
//     function: Func,
//     basic_block: BasicBlock,
// }

pub fn compile_prog(ast: NodePtr, print_tac: bool) -> String {
    debug!("starting compile at {:?}", ast);

    let intermediate_code = tac::transpile_ast(ast);
    let intermediate_code = optimize(intermediate_code);

    if print_tac {
        println!(
            "{}",
            intermediate_code
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    //
    // run code gen
    MipsCompiler::default().compile(intermediate_code).unwrap()
}
