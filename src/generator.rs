use std::collections::VecDeque;

use crate::{node::Node, token::{DataType, NumLiteral}};

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

        if let Node::Define { func_name, func_type, body } = node.clone() {
            if let Some(return_type) = Self::codify(Node::DataType { 
                types: VecDeque::from([*func_type.last()?])
            }) {
                let mut function = if func_name == "main" {
                    "int main() {\n".to_string()
                } else {
                    format!("{return_type} {func_name}(Variable **args, VarList *lst)\n {{")
                };
                for _i in 0..(func_type.len() - 1) {
                    todo!();
                }
                for stmt in body {
                    if let Some(code) = Self::codify(stmt) {
                        function = format!("{function}{code}");
                    } else {
                        return None;
                    }
                }
                if func_name == "main" {
                    function = format!("{}return 0;\n", function);
                }
                function = format!("{}}}\n", function);
                return Some(function);
            }
        }

        if let Node::Return { expr } = node.clone() {
            if let Some(expr) = Self::codify(*expr) {
                return Some(format!("return {expr};\n"));
            }
        }

        if let Node::Funcall { func_name, func_type, in_place_params, pipe } = node.clone() {
            let mut func_name = func_name;
            let mut func_type = func_type;
            let mut in_place_params = in_place_params;
            let mut pipe = pipe;
            let mut funcall = "{\n".to_string();
            funcall = format!("{}VarList *_begin_list = NULL;\n", funcall);
            loop {
                funcall = format!("{funcall}{{\n");
                for i in 0..in_place_params.len() {
                    let NumLiteral::IntLiteral { value } = in_place_params[i];
                    funcall = format!("{}Variable *_param{i} = {value};\n", funcall);
                }
                funcall = format!("{}Variable *_params[] = {{", funcall);
                if !in_place_params.is_empty() {
                    funcall = format!("{}_param0", funcall);
                    for i in 1..in_place_params.len() {
                        funcall = format!("{}, _param{i}", funcall);
                    }
                    funcall = format!("{funcall}, ");
                }
                funcall = format!("{}NULL}};\n", funcall);
                funcall = format!("{}Variable *_res = {func_name}(_params, _begin_list);\n", funcall);
                for _i in 0..(func_type.len() - 1 - in_place_params.len()) {
                    funcall = format!("{}var_dequeue(&_begin_list);\n", funcall);
                }
                funcall = format!("{}if (_res != NULL) {{\nvar_enqueue(&_begin_list, _res);\n}}\n", funcall);
                funcall = format!("{funcall}}}\n");
                if let Some(node) = pipe.clone() {
                    if let Node::Funcall { func_name: f_n, func_type: f_t, in_place_params: i_p_p, pipe: p } = *node {
                        func_name = f_n;
                        func_type = f_t;
                        in_place_params = i_p_p;
                        pipe = p;
                    }
                } else {
                    break;
                }
            }
            funcall = format!("{}var_delete(_begin_list);\n", funcall);
            funcall = format!("{funcall}}}\n");
            return Some(funcall);
        }

        None
    }

    pub fn generate(&mut self) -> String {
        let mut result = String::from("#include <stdlib.h>\n");
        result = format!("{result}#include \"flwrstdlib.h\"\n");
        
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
