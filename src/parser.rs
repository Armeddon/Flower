use std::collections::{VecDeque, HashMap};

use crate::token::{ Token, Keyword, DataType };
use crate::node::{Node, Pipe};

pub struct Parser {
    tokens: VecDeque<Token>,
    functions: HashMap<String, Vec<DataType>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { 
            tokens: tokens.try_into().unwrap(),
            functions: {
                let mut map = HashMap::new();
                map.insert("readInt".to_string(), Vec::from([DataType::Unit]));
                map.insert("println".to_string(), Vec::from([
                    DataType::Int,
                    DataType::Unit,
                ]));
                map.insert("add".to_string(), Vec::from([
                    DataType::Int,
                    DataType::Int,
                    DataType::Int,
                ]));
                map.insert("identity".to_string(), Vec::from([
                    DataType::Int,
                    DataType::Int,
                ]));
                map
            },
        }
    }

    fn peek(&self, n: usize) -> Option<Token> {
        if n >= self.tokens.len() {
            None
        } else {
            Some(self.tokens[n].clone())
        }
    }

    fn consume(&mut self, n: usize) {
        for _ in 0..n {
            self.tokens.pop_front();
        }
    }

    fn parse_expr(&self, n: usize) -> Option<(Node, usize)>{
        if let Some(Token::Numliteral { literal }) = self.peek(n) {
            return Some((Node::NumLiteral { literal }, 1));
        }
        if let Some(Token::DataType { data_type }) = self.peek(n) {
            if let Some(Token::TypeArrow) = self.peek(n + 1) {
                if let Some((Node::DataType { mut types }, tokens)) = self.parse_expr(n + 2) {
                    return Some((Node::DataType { 
                        types: {
                            types.push_front(data_type);
                            types
                        }
                    }, tokens + 2));
                }
            } else {
                return Some((Node::DataType { types: VecDeque::from([data_type]) }, 1));
            }
        }

        None
    }

    fn parse_stmt(&mut self, n: usize) -> Option<(Node, usize)> {
        if let Some(Token::Keyword { keyword }) = self.peek(n) {
            match keyword {
                Keyword::Exit => {
                    if let Some((expr, tokens)) = self.parse_expr(n + 1) {
                        return Some((Node::Exit { expr: Box::from(expr) }, tokens + 1));
                    }
                },
                Keyword::Define => {
                    if let Some(Token::Identifier { name }) = self.peek(n + 1) {
                        if let Some(Token::SpecialArrow) = self.peek(n + 2) {
                            if let Some((Node::DataType { types }, type_tokens)) = self.parse_expr(n + 3) {
                                if let Some(Token::SpecialArrow) = self.peek(n + 3 + type_tokens){
                                    let mut stmts = vec![];
                                    let mut cur = n + 3 + type_tokens + 1;
                                    loop {
                                        if let Some(Token::EndArrow) = self.peek(cur) {
                                            self.functions.insert(name.clone(), types.clone().try_into().unwrap());
                                            return Some((Node::Define {
                                                func_name: name,
                                                func_type: types.try_into().unwrap(),
                                                body: stmts,
                                            }, cur - n + 1));
                                        }
                                        if self.peek(cur).is_none() {
                                            return None;
                                        }
                                        if let Some((stmt, tokens)) = self.parse_stmt(cur) {
                                            stmts.push(stmt);
                                            cur += tokens;
                                        } else {
                                            return None;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        if let Some(Token::Identifier { name }) = self.peek(n) {
            let mut cur = n + 1;
            let mut in_place_params = Vec::new();
            while let Some(token) = self.peek(cur) {
                match token {
                    Token::Numliteral { literal } => {
                        in_place_params.push(literal);
                    },
                    Token::PipeArrow => {
                        if let Some((stmt, tokens)) = self.parse_stmt(cur + 1) {
                            return Some((Node::Funcall {
                                func_name: name.clone(),
                                func_type: if let Some(types) = self.functions.get(&name) {
                                    types.clone() } else { return None; },
                                in_place_params,
                                pipe: Some(Box::from(stmt)),
                                pipe_type: Some(Pipe::Normal),
                            }, cur - n + tokens + 1))
                        }
                    },
                    Token::PreserveArrow => {
                        if let Some((stmt, tokens)) = self.parse_stmt(cur + 1) {
                            return Some((Node::Funcall {
                                func_name: name.clone(),
                                func_type: if let Some(types) = self.functions.get(&name) {
                                    types.clone() } else { return None },
                                in_place_params,
                                pipe: Some(Box::from(stmt)),
                                pipe_type: Some(Pipe::Preserve),
                            }, cur - n + tokens + 1));
                        }
                    },
                    _ => {
                        return Some((Node::Funcall { 
                            func_name: name.clone(),
                            func_type: if let Some(types) = self.functions.get(&name) {
                                types.clone() } else { return None; },
                            in_place_params,
                            pipe: None,
                            pipe_type: None,
                        }, cur - n));
                    },
                }
                cur += 1;
            }
            
        }

        if let Some((expr, tokens)) = self.parse_expr(n) {
            return Some((Node::Return { expr: Box::from(expr) }, tokens));
        }

        None
    }

    pub fn parse(&mut self) -> Option<Vec<Node>> {
        let mut stmts = Vec::new();
        while let Some((stmt, tokens)) = self.parse_stmt(0) {
            self.consume(tokens);
            stmts.push(stmt);
        }
        if self.peek(0).is_some() {
            None
        } else {
            Some(stmts)
        }
    }
}
