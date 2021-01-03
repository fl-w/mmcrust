use super::tac::Address;
use std::collections::HashMap;

pub struct Frame {
    formals: usize,
    offlst: Vec<usize>,
    locals: usize,
    maxargs: Box<usize>,
}

pub struct Level {
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

pub struct SymbolLayer {
    store: HashMap<String, Symbol>,
    num_of_vars: usize,
    func: parser::Func,
}

pub struct SymbolTable {
    store: HashMap<String, SymbolLayer>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, addr: &Address) -> &str { "" }
}
