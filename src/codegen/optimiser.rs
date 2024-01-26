use super::tac::{Addr, Constant, Program, Tac, TacKind};
use itertools::Itertools;
use log::{debug, trace};
use parser::*;
use std::collections::HashMap;

use BinOp::{Add, Div, Mul, Sub};

#[derive(Default)]
pub struct BasicBlock(Program);

impl std::fmt::Debug for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

impl From<Program> for BasicBlock {
    fn from(tacs: Program) -> Self { Self(tacs) }
}

impl BasicBlock {
    pub fn new(prog: Program) -> Self { Self(prog) }

    pub fn into_code(self) -> Program { self.0 }

    pub fn code(&self) -> &Program { &self.0 }

    pub fn code_mut(&mut self) -> &mut Program { &mut self.0 }

    pub fn optimize(&mut self) {
        let mut check = false;
        loop {
            let size = self.code().len();

            self.fold_constants();
            self.propagate_copy();
            self.elim_common_binops();
            self.remove_dead_code();

            if size == self.code().len() {
                // stop optimizing if no change was made
                if check {
                    break;
                } else {
                    check = true;
                }
            } else if check {
                check = false;
            }
        }

        trace!(
            "before ordering temps: {}",
            self.code()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join("\n")
        );

        self.order_tempories();
    }

    fn order_tempories(&mut self) {
        let mut order: HashMap<usize, usize> = HashMap::new();
        let mut index = 0;

        self.code_mut()
            .iter_mut()
            .map(|tac| tac.ref_addr_mut())
            .flatten()
            .for_each(|addr| {
                if let Addr::Temporary(ref mut i) = addr {
                    if let Some(replacement) = order.get(&i) {
                        *i = *replacement;
                    } else {
                        order.insert(*i, index);
                        *i = index;
                        index += 1;
                    }
                }
            })
    }

    fn fold_constants(&mut self) {
        self.code_mut().iter_mut().for_each(|tac| {
            if let TacKind::BinOp(ref op, ref left, ref right) = tac.kind {
                let src = match (op, left, right) {
                    (Add | Sub, a, Addr::Const(Constant::Int32(0)))
                    | (Add | Sub, Addr::Const(Constant::Int32(0)), a)
                    | (Mul | Div, a, Addr::Const(Constant::Int32(1)))
                    | (Mul | Div, Addr::Const(Constant::Int32(1)), a) => a.clone(),

                    (
                        Add | Sub | Div | Mul,
                        Addr::Const(Constant::Int32(a)),
                        Addr::Const(Constant::Int32(b)),
                    ) => Addr::Const(Constant::Int32(match op {
                        Add => a + b,
                        Sub => a - b,
                        Mul => a * b,
                        Div => a / b,
                        _ => unreachable!(),
                    })),

                    _ => return,
                };

                tac.kind = TacKind::Store(src);
            }
        });
    }

    fn remove_dead_code(&mut self) {
        println!(
            "code: {}",
            self.0
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join("\n")
        );
        self.0 = self
            .code_mut()
            .iter_mut()
            .rfold(Vec::new(), |mut code: Vec<Tac>, kind| {
                let t = code.iter().any(|tac| tac.has_src(&kind.target));

                println!("{} {} code: {:?}", kind.target, t, code);

                if matches!(kind.target, Addr::Name(_)) || t {
                    println!("inserting {:?}", kind);
                    code.insert(0, kind.clone());
                }

                code
            });
    }

    fn elim_common_binops(&mut self) {
        let code = self.code_mut();

        let common_expr = code.iter().enumerate().fold(
            HashMap::new(),
            |mut common_expr: HashMap<TacKind, (Vec<usize>, Tac)>, (index, tac)| {
                if let TacKind::BinOp(_, _, _) = tac.kind {
                    if let Some((arr, _)) = common_expr.get_mut(&tac.kind) {
                        arr.push(index);
                    } else {
                        let tac = tac.clone();
                        common_expr.insert(tac.kind.clone(), (vec![index], tac));
                    }
                }

                common_expr
            },
        );

        for (_, (indexes, common_expr)) in common_expr {
            let expr = common_expr.clone_to(Addr::Temporary(code.len()));

            for index in indexes {
                let tac = code.get_mut(index).unwrap();

                if let TacKind::BinOp(op, left, right) = &tac.kind {
                    *tac = expr.clone_to(tac.target.clone());
                }
            }
        }
    }

    fn propagate_copy(&mut self) {
        let mut copies: HashMap<Addr, Addr> = HashMap::new();
        for code in self.code_mut().iter_mut() {
            match code.kind {
                TacKind::BinOp(_, ref mut left, ref mut right) => {
                    for src in vec![left, right] {
                        if let Some(copy) = copies.get(&src) {
                            *src = copy.clone();
                        }
                    }
                }

                TacKind::UnaryOp(_, ref mut src) => {
                    if let Some(copy) = copies.get(&src) {
                        *src = copy.clone();
                    }
                }

                TacKind::Store(ref mut src) => {
                    if let Some(copy) = copies.get(&src) {
                        *src = copy.clone();
                    }

                    copies.insert(code.target.clone(), src.clone());
                }

                _ => (),
            }
        }
    }
}

pub fn into_basic_blocks(prog: Program) -> Vec<BasicBlock> {
    let mut last = 0;
    let mut blocks = Vec::new();

    for leader in prog
        .iter()
        .enumerate()
        .filter(|(_, t)| matches!(t.kind, TacKind::BranchTarget | TacKind::CBranch(_, _, _) ))
        .map(|(i, _)| i)
    {
        println!("index: {}..{}", last, leader);
        blocks.push(prog[last..leader].to_vec());
        last = leader;
    }

    blocks.push(prog[last..prog.len()].to_vec());

    blocks
        .into_iter()
        .filter(|v| !v.is_empty())
        .map_into()
        .collect::<Vec<BasicBlock>>()
}

pub fn optimize(prog: Program) -> Program {
    debug!("starting optimisation with {:#?}", prog);

    let mut blocks = into_basic_blocks(prog);

    debug!("blocks :{:#?}", blocks);

    //
    // machine-independent inta-block optimization
    //
    blocks.iter_mut().for_each(|block| block.optimize()); // optimize blocks

    //
    // TODO: global optimization
    //

    blocks
        .into_iter()
        .map(BasicBlock::into_code)
        .flatten()
        .collect::<Program>()
}
