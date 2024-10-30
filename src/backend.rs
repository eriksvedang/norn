/// Make it easy to swap out the whole compiler backend (e.g. to use
/// LLVM instead of Cranelift).
pub trait Backend {}
