#![allow(dead_code)]

use crate::defs::{FunctionDef, StructDef};
use crate::sympath::SymPath;
use std::collections::HashMap;
use std::env::SplitPaths;

pub struct Builder {
    functions: HashMap<SymPath, FunctionDef>,
    structs: HashMap<SymPath, StructDef>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            functions: Default::default(),
            structs: Default::default(),
        }
    }

    pub fn AddFunction(&mut self, path: SymPath, function: FunctionDef) {
        self.functions.insert(path, function);
    }
}
