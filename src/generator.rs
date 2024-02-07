use std::collections::VecDeque;

use crate::{node::{Node, Pipe}, token::{DataType, NumLiteral}};

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

        if let Node::DataType { types } = node.clone() {
            match types.get(0)? {
                DataType::Int => return Some("Int".to_string()),
                DataType::Unit => return Some("Unit".to_string()),
            }
        }

        if let Node::Define { func_name, func_type, body } = node.clone() {
            if let Some(_) = Self::codify(Node::DataType { 
                types: VecDeque::from([*func_type.last()?])
            }) {
                let mut function = format!(
                    "static Variable *flwr_{func_name}(Variable **args, VarList *lst) {{\n"
                );
                function = format!(
                    "{}var_take_pextend(&lst, args, min(var_len(args), {}));\n",
                    function, func_type.len()-1
                );
                function = format!("{}Variable *_result = NULL;\n", function);
                for i in 0..(func_type.len() - 1) {
                    function = format!(
                        "{}Variable *_arg{i} = var_get(lst, {i});\n",
                        function
                    );
                }
                for i in 0..(func_type.len() - 1) {
                    function = format!(
                        "{}if (var_get_type(_arg{i}) != {}) {{\n",
                        function, Self::codify(
                            Node::DataType { 
                                types: vec![func_type[i]].try_into().unwrap()
                            })?
                    );
                    function = format!(
                        "{}var_take_delete(&lst, min(var_len(args), {}));\nreturn NULL;\n}}\n",
                        function, func_type.len()-1
                    );
                }
                for stmt in body {
                    if let Some(code) = Self::codify(stmt) {
                        function = format!("{function}{code}");
                    } else {
                        return None;
                    }
                }
                function = format!(
                    "{}var_take_delete(&lst, min(var_len(args), {}));\n",
                    function, func_type.len()-1
                );
                function = format!("{}return _result;\n", function);
                function = format!("{}}}\n", function);
                return Some(function);
            }
        }

        if let Node::Return { expr } = node.clone() {
            if let Some(expr) = Self::codify(*expr) {
                let mut ret = format!("int *_result_value = malloc(sizeof(int));\n");
                ret = format!("{}*_result_value = {expr};\n", ret);
                ret = format!("{}_result = var_create(Int, _result_value);\n", ret);
                return Some(ret);
            }
        }

        if let Node::Funcall { this_func_type, func_name, func_type, in_place_params, pipe, pipe_type } = node.clone() {
            let mut func_name = func_name;
            let mut func_type = func_type;
            let mut in_place_params = in_place_params;
            let mut pipe = pipe;
            let mut pipe_type = pipe_type;
            let mut funcall = "{\n".to_string();
            funcall = format!(
                "{}VarList *_begin_list = var_take_copy(lst, {});\n",
                funcall, this_func_type.len()-1
            );
            loop {
                funcall = format!("{funcall}{{\n");
                for i in 0..in_place_params.len() {
                    let NumLiteral::IntLiteral { value } = in_place_params[i];
                    funcall = format!("{}int *_param{i}_value = malloc(sizeof(int));\n", funcall);
                    funcall = format!("{}*_param{i}_value = {value};\n", funcall);
                    funcall = format!("{}Variable *_param{i} = var_create(Int, _param{i}_value);\n", funcall);
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
                funcall = format!(
                    "{}Variable *_res = flwr_{func_name}(_params, _begin_list);\n",
                    funcall
                );
                for i in 0..in_place_params.len() {
                    funcall = format!("{}var_free(_param{i});\n", funcall);
                }
                if pipe_type != Some(Pipe::Preserve) {
                    for _i in 0..(func_type.len() - 1 - in_place_params.len()) {
                        funcall = format!("{}var_dequeue(&_begin_list);\n", funcall);
                    }
                }
                funcall = format!("{}if (_res != NULL) {{\n", funcall);
                if pipe_type == Some(Pipe::Prepend) {
                    funcall = format!("{}var_prepend(&_begin_list, _res);\n}}\n", funcall);
                } else {
                    funcall = format!("{}var_enqueue(&_begin_list, _res);\n}}\n", funcall);
                }
                funcall = format!("{funcall}}}\n");
                if let Some(node) = pipe.clone() {
                    if let Node::Funcall { this_func_type: _, func_name: f_n, func_type: f_t, in_place_params: i_p_p, pipe: p, pipe_type: p_t } = *node {
                        func_name = f_n;
                        func_type = f_t;
                        in_place_params = i_p_p;
                        pipe = p;
                        pipe_type = p_t;
                    }
                } else {
                    break;
                }
            }
            funcall = format!("{}_result = var_cpy(var_get(_begin_list, 0));\n", funcall);
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

        result = format!("{result}{}", Self::add_main());

        result
    }

    fn add_main() -> String {
        let mut main = "int main(void) {\n".to_string();
        main = format!("{}flwr_main(NULL, NULL);\n", main);
        main = format!("{}return 0;\n", main);
        main = format!("{}}}", main);
        main
    }
}
