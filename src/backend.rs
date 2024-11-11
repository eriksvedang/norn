use crate::{ast::AstNode, ty::Ty};

/// Make it easy to swap out the whole compiler backend (e.g. to use
/// LLVM instead of Cranelift).
pub trait Backend {
    type FunctionHandle;

    fn define_function(&mut self, name: &str, func_ty: Ty, ast: AstNode) -> Self::FunctionHandle;
    fn call_function(&self, function_handle: &Self::FunctionHandle) -> i32;
}
