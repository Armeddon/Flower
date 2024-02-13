use crate::token::{Literal, DataType };

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
    pub in_place_params: Vec<Literal>,
    pub pipe: Option<Box<Funcall>>,
    pub pipe_type: Option<Pipe>,
    pub this_func_type: Vec<DataType>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
}

#[derive(Debug, Clone)]
pub enum Node {
    Define(Define),
    Return(Box<Expr>),
    Funcall(Funcall),
}

impl From<&Expr> for DataType {
    fn from(expr: &Expr) -> Self {
        match expr.clone() {
            Expr::Literal(lit) => lit.clone().into(),
        }
    }
}

impl Expr {
    pub fn c_type_repr(&self) -> String {
        match self.clone() {
            Expr::Literal(lit) => lit.c_type_repr()
        }
    }
}
