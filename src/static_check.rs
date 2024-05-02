use std::collections::VecDeque;

use crate::node::{Define, If, Funcall, Node, Return};
use crate::token::{DataType};

pub struct CheckResult {
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

pub fn static_check(nodes: Vec<Node>) -> CheckResult {
    let mut warnings = Vec::new();
    let mut errors = Vec::new();

    for node in nodes {
        let CheckResult { warnings: mut w, errors: mut e} = node_check(node);
        warnings.append(w.as_mut());
        errors.append(e.as_mut());
    }

    CheckResult{warnings, errors}
}

fn node_check(node: Node) -> CheckResult {
    match node {
        Node::Return(ret) => {
            return_check(ret)
        },
        Node::If(if_stmt) => {
            if_check(if_stmt)
        },
        Node::Define(define) => {
            define_check(define)
        },
        Node::Funcall(funcall) => {
            funcall_check(funcall)
        },
    }
}

fn return_check(ret: Return) -> CheckResult {
    if ret.return_type.is_none() {
        return CheckResult {
            warnings: Vec::new(),
            errors: vec!["An expression outside a function!".to_string()]
        };
    }
    if DataType::from(ret.expr.as_ref()).inherits(&ret.return_type.unwrap()) {
        return CheckResult{
            warnings: Vec::new(),
            errors: vec!["The type of the expression doesn't match the return type!".to_string()]
        };
    }

    CheckResult {
        warnings: Vec::new(),
        errors: Vec::new()
    }
}

fn if_check(if_stmt: If) -> CheckResult {
    CheckResult {
        warnings: Vec::new(),
        errors: Vec::new()
    }
}

fn define_check(define: Define) -> CheckResult {
    let Define { func_name: _, func_type, body } = define;
    let mut types = func_type.clone();
    types.truncate(func_type.len()-1);
    let mut types: VecDeque<_> = types.into();
    for node in body {
        match node {
           _ => todo!(),
        }
    }

    CheckResult {
        warnings: Vec::new(),
        errors: Vec::new()
    }
}

fn funcall_check(define: Funcall) -> CheckResult {
    todo!();
    CheckResult {
        warnings: Vec::new(),
        errors: Vec::new()
    }
}
