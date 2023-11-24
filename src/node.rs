use crate::token::NumLiteral;

#[derive(Debug, Clone)]
pub enum Node {
    NumLiteral {
        literal: NumLiteral,
    },
    Exit {
        expr: Box<Node>,
    },
}

