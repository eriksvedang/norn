// An example of how to compile math expressions using Gryphon

use norn::{
    ast::AstNode, backends::cranelift::CraneliftBackend, builder::Builder,
    parameter_list::ParameterList, sympath::SymPath,
};

#[allow(dead_code)]
pub extern "C" fn print(x: i32) {
    println!("{}", x);
}

#[allow(dead_code)]
pub extern "C" fn foo(x: i32) -> i32 {
    println!("foo was called (x={})", x);
    x * 2
}

pub fn main() {
    let backend = CraneliftBackend::new();
    let mut builder = Builder::new(backend);

    builder.define_function(
        SymPath::from_str("say_hello"),
        ParameterList(vec![]),
        AstNode::Do(vec![AstNode::Call(SymPath::from_str("print"), vec![])]),
    );
}
