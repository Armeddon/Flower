use std::collections::VecDeque;

use crate::token::{NumLiteral, DataType };

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
        name: String,
        func_type: Vec<DataType>,
        body: Vec<Node>,
    },
}

