// An example of how to compile math expressions using Norn

use std::collections::HashMap;

use norn::{
    ast::AstNode, backends::cranelift::CraneliftBackend, builder::Builder,
    parameter_list::ParameterList, sympath::SymPath,
};

#[allow(dead_code)]
pub extern "C" fn print_int(x: i32) {
    println!("{}", x);
}

#[allow(dead_code)]
pub extern "C" fn square(x: i32) -> i32 {
    x * x
}

pub fn main() {
    let c_functions = HashMap::from_iter([
        (String::from("print_int"), print_int as *const u8),
        (String::from("square"), square as *const u8),
    ]);

    let backend = CraneliftBackend::new(c_functions);
    let mut builder = Builder::new(backend);

    builder.define_function(
        SymPath::from_str("print_42"),
        ParameterList(vec![]),
        AstNode::Do(vec![AstNode::Call(
            SymPath::from_str("print_int"),
            vec![AstNode::Constant(42)],
        )]),
    );

    let answer = builder.call_without_arguments(&SymPath::from_str("print_42"));
    println!("answer = {:?}", answer);
}
