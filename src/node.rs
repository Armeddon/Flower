use std::collections::VecDeque;

use crate::token::{NumLiteral, DataType };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pipe {
    Normal,
    Preserve,
    Prepend,
}

#[derive(Debug, Clone)]
pub enum Node {
    NumLiteral {
        literal: NumLiteral,
    },
    Exit {
        expr: Box<Node>,
    },
    DataType {
        types: VecDeque<DataType>,
    },
    Define {
        func_name: String,
        func_type: Vec<DataType>,
        body: Vec<Node>,
    },
    Return {
        expr: Box<Node>,
    },
    Funcall {
        func_name: String,
        func_type: Vec<DataType>,
        in_place_params: Vec<NumLiteral>,
        pipe: Option<Box<Node>>,
        pipe_type: Option<Pipe>,
    },
}

