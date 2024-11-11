#![allow(dead_code)]

use crate::ast::AstNode;
use crate::backend::Backend;
use crate::defs::{FunctionDef, StructDef};
use crate::parameter_list::ParameterList;
use crate::sympath::SymPath;
use crate::ty::Ty;
use std::collections::HashMap;

pub struct Builder<B: Backend> {
    backend: B,
    functions: HashMap<SymPath, FunctionDef<B::FunctionHandle>>,
    structs: HashMap<SymPath, StructDef>,
}

impl<B: Backend> Builder<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            functions: Default::default(),
            structs: Default::default(),
        }
    }

    pub fn define_function(&mut self, path: SymPath, _params: ParameterList, body: AstNode) {
        let handle = self
            .backend
            .define_function(&path.to_string(), Ty::I32, body);
        self.functions.insert(path, FunctionDef { handle });
    }

    // TODO: define_anonymous_function

    pub fn define_struct(&mut self, path: SymPath, struct_def: StructDef) {
        self.structs.insert(path, struct_def);
    }

    pub fn call_without_arguments(&self, sympath: &SymPath) -> Option<i32> {
        if let Some(func) = self.functions.get(sympath) {
            Some(self.backend.call_function(&func.handle))
        } else {
            None
        }
    }
}
