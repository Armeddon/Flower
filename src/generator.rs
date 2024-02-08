use std::collections::VecDeque;

use crate::{
    node::{Define, Funcall, Expr, Node, Pipe},
    token::{DataType, NumLiteral}
};

pub fn generate(nodes: Vec<Node>) -> String {
    let mut generator = Generator::new(nodes);
    let mut result = Generator::add_includes();

    while let Some(node) = generator.peek(0) {
        let Some(code) = Generator::codify(&node) else {
            break;
        };
        generator.consume(1);
        result = format!("{result}{code}");
    }

    result = format!("{result}{}", Generator::add_main());

    result
}

struct Generator {
    nodes: VecDeque<Node>
}

impl Generator {
    fn new(nodes: Vec<Node>) -> Self {
        Self {
            nodes: nodes.try_into().unwrap(),
        }
    }

    fn codify(node: &Node) -> Option<String> {
        match node {
            Node::Return(expr) => Self::try_codify_return(expr.as_ref()),
            Node::Define(define) => Self::try_codify_define(define),
            Node::Funcall(funcall) => Self::try_codify_funcall(funcall),
        }
    }

    fn codify_data_type(data_type: DataType) -> String {
        data_type.into()
    }

    fn try_codify_return(expr: &Expr) -> Option<String> {
        let Some(str_expr) = Self::try_codify_expr(expr) else {
            return None;
        };
        let mut ret = format!("{} *_result_value = malloc(sizeof(int));\n",
            DataType::from(expr).c_string()
            );
        ret = format!("{}*_result_value = {str_expr};\n", ret);
        ret = format!("{}_result = var_create({}, _result_value);\n",
            ret, String::from(DataType::from(expr)));
        Some(ret)
    }

    fn try_codify_define(define: &Define) -> Option<String> {
        let Define{func_name, func_type, body } = define.clone();
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
                function, Self::codify_data_type(
                    func_type[i])
                );
            function = format!(
                "{}var_take_delete(&lst, min(var_len(args), {}));\nreturn NULL;\n}}\n",
                function, func_type.len()-1
                );
        }
        for stmt in body {
            if let Some(code) = Self::codify(&stmt) {
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
        Some(function)
    }

    fn try_codify_funcall(funcall: &Funcall) -> Option<String> {
        let Funcall {this_func_type, func_name, func_type, in_place_params, pipe, pipe_type } = funcall.clone();
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
                let NumLiteral::IntLiteral(value) = in_place_params[i];
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
                let Funcall {this_func_type: _, func_name: f_n, func_type: f_t, in_place_params: i_p_p, pipe: p, pipe_type: p_t } = *node;
                func_name = f_n;
                func_type = f_t;
                in_place_params = i_p_p;
                pipe = p;
                pipe_type = p_t;
            } else {
                break;
            }
        }
        funcall = format!("{}_result = var_cpy(var_get(_begin_list, 0));\n", funcall);
        funcall = format!("{}var_delete(_begin_list);\n", funcall);
        funcall = format!("{funcall}}}\n");
        return Some(funcall);
    }

    fn add_includes() -> String {
        let includes = String::from("#include <stdlib.h>\n");
        format!("{includes}#include \"flwrstdlib.h\"\n")
    }

    fn add_main() -> String {
        let mut main = "int main(void) {\n".to_string();
        main = format!("{}flwr_main(NULL, NULL);\n", main);
        main = format!("{}return 0;\n", main);
        main = format!("{}}}", main);
        main
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
    fn try_codify_expr(expr: &Expr) -> Option<String> { 
        match *expr {
            Expr::NumLiteral(lit) => Some(Self::codify_literal(lit)),
        }
    }

    fn codify_literal(literal: NumLiteral) -> String {
        format!("{literal}")
    }
}
