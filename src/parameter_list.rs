#![allow(dead_code)]

use crate::ty::Ty;

pub struct ParameterList(pub Vec<(String, Ty)>);
