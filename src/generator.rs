use std::collections::VecDeque;

use crate::{node::Node, token::DataType};

pub struct Generator {
    nodes: VecDeque<Node>
}

impl Generator {
    pub fn new(nodes: Vec<Node>) -> Self {
        Self {
            nodes: nodes.try_into().unwrap(),
        }
    }

    fn peek(&self, n: usize) -> Option<Node> {
        if n >= self.nodes.len() {
            None
        } else {
            Some(self.nodes[n].clone())
        }
    }

    fn consume(&mut self, n: usize) {
        for _ in 0..n {
            self.nodes.pop_front();
        }
    }

    fn codify(node: Node) -> Option<String> {
        if let Node::NumLiteral { literal } = node.clone() {
            return Some(format!("{literal}"));
        }

        if let Node::Exit { expr } = node.clone() {
            if let Some(code) = Self::codify(*expr) {
                return Some(format!("exit({code});\n"));
            }
        }

        if let Node::DataType { types } = node.clone() {
            match types.get(0)? {
                DataType::Int => return Some("int".to_string()),
                DataType::Unit => return Some("void".to_string()),
            }
        }

        if let Node::Define { name, func_type, body } = node.clone() {
            if let Some(return_type) = Self::codify(Node::DataType { 
                types: func_type.try_into().unwrap()
            }) {
                let mut function = format!("{return_type} {name}() {{\n");
                for stmt in body {
                    if let Some(code) = Self::codify(stmt) {
                        function = format!("{function}{code}");
                    } else {
                        return None;
                    }
                }
                function = format!("{}}}\n", function);
                return Some(function);
            }
        }

        None
    }

    pub fn generate(&mut self) -> String {
        let mut result = String::from("#include <stdlib.h>\n");
        
        while let Some(node) = self.peek(0) {
            let Some(code) = Self::codify(node) else {
                break;
            };
            self.consume(1);
            result = format!("{result}{code}");
        }

        result
    }
}
