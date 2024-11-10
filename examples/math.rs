// An example of how to compile math expressions using Gryphon

use norn::{
    ast::AstNode, backends::cranelift::CraneliftBackend, builder::Builder, sympath::SymPath,
};

pub fn main() {
    let _backend = CraneliftBackend::new();
    let mut builder = Builder::new();

    builder.AddFunction(SymPath::from_str("hello"), AstNode::Do(vec![]));
}
