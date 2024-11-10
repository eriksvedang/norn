#![allow(dead_code)]

use crate::ast::AstNode;
use crate::backend::Backend;
use crate::defs::{FunctionDef, FunctionHandle, StructDef, StructHandle};
use crate::parameter_list::ParameterList;
use crate::sympath::SymPath;
use std::collections::HashMap;

pub struct Builder<B> {
    backend: B,
    functions: HashMap<SymPath, FunctionDef>,
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

    pub fn define_function(
        &mut self,
        path: SymPath,
        _params: ParameterList,
        _body: AstNode,
    ) -> FunctionHandle {
        self.backend.define_function("sfdfssfs");
        self.functions.insert(path, FunctionDef {});
        FunctionHandle(self.functions.len() - 1)
    }

    // TODO: define_anonymous_function

    pub fn define_struct(&mut self, path: SymPath, struct_def: StructDef) -> StructHandle {
        self.structs.insert(path, struct_def);
        StructHandle(self.structs.len() - 1)
    }
}
