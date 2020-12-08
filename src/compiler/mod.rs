pub mod tac;

use std::collections::HashMap;
use Infix::{Add, Divide, Multiply, Subtract};

use itertools::Itertools;
use log::{debug, trace};
use parser::*;
use tac::Address::*;

use self::tac::{Address, BinOp, IntermediateCode, ThreeAddressCode};

struct CodeGen {
    symbol_table: HashMap<char, String>,
}

#[derive(Debug, Default)]
pub struct BasicBlock(IntermediateCode);

impl From<IntermediateCode> for BasicBlock {
    fn from(tacs: IntermediateCode) -> Self { Self(tacs) }
}

impl BasicBlock {
    pub fn new(code: IntermediateCode) -> Self { Self(code) }

    pub fn into_code(self) -> IntermediateCode { self.0 }

    pub fn code(&self) -> &IntermediateCode { &self.0 }

    pub fn code_mut(&mut self) -> &mut IntermediateCode { &mut self.0 }

    pub fn optimize(&mut self) {
        let mut check = false;
        loop {
            let size = self.code().len();

            self.fold_constants();
            self.propagate_copy();
            self.elim_common_expr();
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
                if let Address::Temporary(ref mut i) = addr {
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
            if let ThreeAddressCode::BinOp(BinOp {
                ref op,
                ref src,
                ref dst,
            }) = tac
            {
                let src = match (op, &src.0, &src.1) {
                    (Add | Subtract, a, Const(0))
                    | (Add | Subtract, Const(0), a)
                    | (Multiply | Divide, a, Const(1))
                    | (Multiply | Divide, Const(1), a) => a.clone(),

                    (Add | Subtract | Divide | Multiply, Const(a), Const(b)) => Const(match op {
                        Add => a + b,
                        Subtract => a - b,
                        Multiply => a * b,
                        Divide => a / b,
                        _ => unreachable!(),
                    }),

                    _ => return,
                };

                *tac = ThreeAddressCode::Copy {
                    dst: dst.clone(),
                    src,
                };
            }
        });
    }

    fn remove_dead_code(&mut self) {
        self.0 = self.code_mut().iter_mut().rev().fold(
            Vec::new(),
            |mut code: Vec<ThreeAddressCode>, inst| {
                if code.is_empty() || code.iter().any(|tac| tac.has_src(inst.dst().unwrap())) {
                    code.insert(0, inst.clone());
                }

                code
            },
        );
    }

    fn elim_common_expr(&mut self) {
        let code = self.code_mut();

        let common_expr = code.iter().enumerate().fold(
            HashMap::new(),
            |mut common_expr: HashMap<BinOp, Vec<usize>>, (index, tac)| {
                if let ThreeAddressCode::BinOp(tac) = tac {
                    if let Some(arr) = common_expr.get_mut(tac) {
                        arr.push(index);
                    } else {
                        common_expr.insert(tac.clone(), vec![index]);
                    }
                }

                common_expr
            },
        );

        for (common_expr, indexes) in common_expr {
            let mut expr = common_expr.clone();
            expr.dst = Address::Temporary(code.len());

            for index in indexes {
                let tac = code.get_mut(index).unwrap();

                if let ThreeAddressCode::BinOp(BinOp { dst, src: _, op: _ }) = tac {
                    *tac = ThreeAddressCode::BinOp(expr.clone_to(dst.clone()));
                }
            }
        }
    }

    fn propagate_copy(&mut self) {
        let mut copies: HashMap<Address, Address> = HashMap::new();
        for code in self.code_mut().iter_mut() {
            match code {
                ThreeAddressCode::BinOp(BinOp {
                    ref mut src,
                    dst: _,
                    op: _,
                }) => {
                    let (ref mut left, ref mut right) = src;
                    for src in vec![left, right] {
                        if let Some(copy) = copies.get(&src) {
                            *src = copy.clone();
                        }
                    }
                }
                ThreeAddressCode::UnaryOp {
                    ref mut src,
                    dst: _,
                    op: _,
                } => {
                    if let Some(copy) = copies.get(&src) {
                        *src = copy.clone();
                    }
                }
                ThreeAddressCode::Copy { dst, ref mut src } => {
                    if let Some(copy) = copies.get(&src) {
                        *src = copy.clone();
                    }

                    copies.insert(dst.clone(), src.clone());
                }
            }
        }
    }
}

pub fn optimize(code: IntermediateCode) -> IntermediateCode {
    let blocks = code
        .into_iter()
        .enumerate()
        .group_by(|(i, tac_code)| *i == 0 || matches!(tac_code, ThreeAddressCode::Label(_)))
        .into_iter()
        .map(|(_, code)| code.map(|(_, tac)| tac).collect::<IntermediateCode>())
        .map(Into::into)
        .collect::<Vec<BasicBlock>>();

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
        .collect::<IntermediateCode>()
}

pub fn compile(ast: NodePtr) {
    let intermediate_code = tac::transpile_ast(ast);

    let intermediate_code = optimize(intermediate_code);

    debug!(
        "{}",
        intermediate_code
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join("\n")
    );
}
