use std::collections::VecDeque;

use crate::node::Node;

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

    fn codify(node: Node) -> Option<(String, usize)> {
        if let Node::NumLiteral { literal } = node {
            return Some((format!("{literal}"), 1));
        }

        if let Node::Exit { expr } = node {
            if let Some((code, nodes)) = Self::codify(*expr) {
                return Some((format!("return {code};\n"), nodes + 1));
            }
        }

        None
    }

    pub fn generate(&mut self) -> String {
        let mut result = String::from("int main (void) {\n");
        
        while let Some(node) = self.peek(0) {
            let Some((code, nodes)) = Self::codify(node) else {
                break;
            };
            self.consume(nodes);
            result = format!("{result}{code}");
        }

        result = format!("{result}}}");

        result
    }
}
