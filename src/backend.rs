/// Make it easy to swap out the whole compiler backend (e.g. to use
/// LLVM instead of Cranelift).
pub trait Backend {
    fn define_function(&mut self, name: &str);
}
