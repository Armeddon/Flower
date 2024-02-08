use std::collections::VecDeque;

use crate::token::{NumLiteral, DataType };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pipe {
    Normal,
    Preserve,
    Prepend,
}

#[derive(Debug, Clone)]
pub struct Define {
    pub func_name: String,
    pub func_type: Vec<DataType>,
    pub body: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Funcall {
    pub func_name: String,
    pub func_type: Vec<DataType>,
    pub in_place_params: Vec<NumLiteral>,
    pub pipe: Option<Box<Node>>,
    pub pipe_type: Option<Pipe>,
    pub this_func_type: Vec<DataType>,
}

#[derive(Debug, Clone)]
pub enum Node {
    NumLiteral(NumLiteral),
    DataType(VecDeque<DataType>),
    Define(Define),
    Return(Box<Node>),
    Funcall (Funcall),
}

