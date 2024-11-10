use crate::sympath::SymPath;

#[derive(Clone, Debug)]
pub enum AstNode {
    Do(Vec<AstNode>),
    Call(SymPath),
    SetLocal(String, Box<AstNode>),
    Variable(String),
    Constant(i32),
}
