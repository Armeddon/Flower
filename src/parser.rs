use std::collections::{VecDeque, HashMap};

use crate::token::{ Token, Keyword, DataType };
use crate::node::{ Node, Pipe };

pub fn parse(tokens: Vec<Token>) -> Option<Vec<Node>> {
    let mut parser = Parser::new(tokens);
    let mut stmts = Vec::new();
    while let Some((stmt, tokens)) = parser.try_parse_stmt(0) {
        parser.consume(tokens);
        stmts.push(stmt);
    }
    if parser.peek(0).is_some() {
        None
    } else {
        Some(stmts)
    }
}

fn std_functions() -> HashMap<String, Vec<DataType>> {
    let mut map = HashMap::new();
    map.insert("readInt".to_string(), Vec::from([DataType::Int]));
    map.insert("println".to_string(), Vec::from([
        DataType::Int,
        DataType::Unit,
    ]));
    map.insert("add".to_string(), Vec::from([
        DataType::Int,
        DataType::Int,
        DataType::Int,
    ]));
    map.insert("id".to_string(), Vec::from([
       DataType::Int,
       DataType::Int,
    ]));
    map 
}

struct Parser {
    tokens: VecDeque<Token>,
    functions: HashMap<String, Vec<DataType>>,
    this_function: Vec<DataType>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { 
            this_function: vec![],
            tokens: tokens.try_into().unwrap(),
            functions: std_functions(),
        }
    }

    fn consume(&mut self, n: usize) {
        for _ in 0..n {
            self.tokens.pop_front();
        }
    }

    fn try_parse_stmt(&mut self, n: usize) -> Option<(Node, usize)> {
        if let Some(kw) = self.try_parse_keyword(n) {
            return Some(kw);
        }
        if let Some(fc) = self.try_parse_funcall(n) {
            return Some(fc);
        }
        if let Some((expr, tokens)) = self.try_parse_expr(n) {
            return Some((Node::Return { expr: Box::from(expr) }, tokens));
        }
        None
    }

    fn try_parse_expr(&self, n:usize) -> Option<(Node, usize)> {
        if let Some(lit) = self.try_parse_literal(n) {
            return Some(lit);
        }
        None
    }

    fn try_parse_literal(&self, n: usize) -> Option<(Node, usize)> {
        if let Some(Token::NumLiteral { literal }) = self.peek(n) {
            return Some((Node::NumLiteral { literal }, 1));
        }
        None
    }

    fn try_parse_funcall(&self, n: usize) -> Option<(Node, usize)> {
        let Token::Identifier { name } = self.peek(n)? else {
            return None;
        };
        let mut cur = n + 1;
        let mut in_place_params = Vec::new();
        while let Some(Token::NumLiteral { literal }) = self.peek(cur) {
            in_place_params.push(literal);
            cur += 1;
        }
        let is_piped = match self.peek(cur) {
            Some(Token::PipeArrow) | 
                Some(Token::PrependArrow) | 
                Some(Token::PreserveArrow) => true,
            _ => false,
        };
        let pipe_funcall = if is_piped { 
            self.try_parse_funcall(cur + 1)
        } else {None};
        let pipe = pipe_funcall.clone().map(|node_with_tokens: (Node, usize)| {
            let (node, _) = node_with_tokens;
            Box::from(node)
        });
        let tokens = pipe_funcall.map(|node_with_tokens: (Node, usize)| {
            let (_, tokens) = node_with_tokens;
            tokens
        }).unwrap_or(0);
        Some((Node::Funcall {
            this_func_type: self.this_function.clone(),
            func_name: name.clone(),
            func_type: if let Some(types) = self.functions.get(&name) {
                types.clone()
            } else { return None; },
            in_place_params,
            pipe,
            pipe_type: {
                match self.peek(cur) {
                    None => None,
                    Some(token) => match token {
                        Token::PipeArrow => Some(Pipe::Normal),
                        Token::PreserveArrow => Some(Pipe::Preserve),
                        Token::PrependArrow => Some(Pipe::Prepend),
                        _ => None,
                    }
                }
            },
        }, cur - n + tokens + if is_piped {1} else {0}))
    }

    fn try_parse_keyword(&mut self, n: usize) -> Option<(Node, usize)> {
        if self.peek(n).is_none() {
            return None;
        }
        let Token::Keyword { keyword } = self.peek(n).unwrap()
        else {
            return None;
        };
        match keyword {
            Keyword::Define => {
                self.try_parse_define(n)
            },
        }
    }

    fn try_parse_define(&mut self, n: usize) -> Option<(Node, usize)> {
        let Token::Identifier { name } = self.peek(n + 1)? else {
            return None;
        };
        if self.peek(n + 2)? != Token::SpecialArrow {
            return None;
        }
        let (Node::DataType { types }, type_tokens) = self.try_parse_data_type(n + 3)? else {
            return None;
        };
        if self.peek(n + 3 + type_tokens)? != Token::SpecialArrow {
            return None;
        }
        let mut stmts = Vec::new();
        let mut cur = n + 3 + type_tokens + 1;
        self.this_function = types.clone().try_into().unwrap();
        loop {
            if self.peek(cur)? == Token::EndArrow {
                break;
            }
            let Some((stmt, tokens)) = self.try_parse_stmt(cur) else {
                return None;
            };
            stmts.push(stmt);
            cur += tokens;
        }
        self.functions.insert(name.clone(), types.clone().try_into().unwrap());
        return Some((Node::Define {
            func_name: name,
            func_type: types.try_into().unwrap(),
            body: stmts,
        }, cur - n + 1));
    }

    fn try_parse_data_type(&self, n: usize) -> Option<(Node, usize)> { 
        let Token::DataType { data_type } = self.peek(n)? else
        {
            return None;
        };

        if self.peek(n + 1).is_none() {
            return Some((Node::DataType { types: VecDeque::from([data_type]) }, 1));
        }
        if self.peek(n + 1).unwrap() != Token::TypeArrow {
            return Some((Node::DataType { types: VecDeque::from([data_type]) }, 1));
        }
        if let Some((Node::DataType { mut types }, tokens)) = self.try_parse_data_type(n + 2) {
            return Some((Node::DataType { 
                types: {
                    types.push_front(data_type);
                    types
                }
            }, tokens + 2));
        }
        None
    }

    fn peek(&self, n: usize) -> Option<Token> {
        if n >= self.tokens.len() {
            None
        } else {
            Some(self.tokens[n].clone())
        }
    }
}
