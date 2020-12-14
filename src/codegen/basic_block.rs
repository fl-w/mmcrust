#[derive(Default)]
pub struct BasicBlock(IntermediateCode);

impl std::fmt::Debug for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.0)
    }
}

impl From<IntermediateCode> for BasicBlock {
    fn from(tacs: IntermediateCode) -> Self { Self(tacs) }
}

impl BasicBlock {
    pub fn new(prog: IntermediateCode) -> Self { Self(prog) }

    pub fn into_code(self) -> IntermediateCode { self.0 }

    pub fn code(&self) -> &IntermediateCode { &self.0 }

    pub fn code_mut(&mut self) -> &mut IntermediateCode { &mut self.0 }

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
            if let Inst::BinOp(ref op, ref left, ref right) = tac.inst {
                let src = match (op, left, right) {
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

                tac.inst = Inst::Copy(src);
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
        self.0 = self.code_mut().iter_mut().rfold(
            Vec::new(),
            |mut code: Vec<ThreeAddressCode>, inst| {
                let t = code.iter().any(|tac| tac.has_src(&inst.dst));

                println!("{} {} code: {:?}", inst.dst, t, code);

                if matches!(inst.dst, Address::Name(_)) || t {
                    println!("inserting {:?}", inst);
                    code.insert(0, inst.clone());
                }

                code
            },
        );
    }

    fn elim_common_binops(&mut self) {
        let code = self.code_mut();

        let common_expr = code.iter().enumerate().fold(
            HashMap::new(),
            |mut common_expr: HashMap<Inst, (Vec<usize>, ThreeAddressCode)>, (index, tac)| {
                if let Inst::BinOp(_, _, _) = tac.inst {
                    if let Some((arr, _)) = common_expr.get_mut(&tac.inst) {
                        arr.push(index);
                    } else {
                        let tac = tac.clone();
                        common_expr.insert(tac.inst.clone(), (vec![index], tac));
                    }
                }

                common_expr
            },
        );

        for (_, (indexes, common_expr)) in common_expr {
            let expr = common_expr.clone_to(Address::Temporary(code.len()));

            for index in indexes {
                let tac = code.get_mut(index).unwrap();

                if let Inst::BinOp(op, left, right) = &tac.inst {
                    *tac = expr.clone_to(tac.dst.clone());
                }
            }
        }
    }

    fn propagate_copy(&mut self) {
        let mut copies: HashMap<Address, Address> = HashMap::new();
        for code in self.code_mut().iter_mut() {
            match code.inst {
                Inst::BinOp(_, ref mut left, ref mut right) => {
                    for src in vec![left, right] {
                        if let Some(copy) = copies.get(&src) {
                            *src = copy.clone();
                        }
                    }
                }

                Inst::UnaryOp(_, ref mut src) => {
                    if let Some(copy) = copies.get(&src) {
                        *src = copy.clone();
                    }
                }

                Inst::Copy(ref mut src) => {
                    if let Some(copy) = copies.get(&src) {
                        *src = copy.clone();
                    }

                    copies.insert(code.dst.clone(), src.clone());
                }

                _ => (),
            }
        }
    }
}

pub fn into_basic_blocks(prog: IntermediateCode) -> Vec<BasicBlock> {
    let mut last = 0;
    let mut blocks = Vec::new();

    for leader in prog
        .iter()
        .enumerate()
        .filter(|(_, t)| matches!(t.inst, Inst::BranchTarget | Inst::CBranch(_, _, _) ))
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

pub fn optimize(prog: IntermediateCode) -> IntermediateCode {
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
        .collect::<IntermediateCode>()
}
