use crate::sympath::SymPath;

#[derive(Clone, Debug)]
pub enum AstNode {
    Do(Vec<AstNode>),
    Call(SymPath, Vec<AstNode>),
    SetLocal(String, Box<AstNode>),
    Variable(String),
    Constant(i32),
}
