use std::collections::VecDeque;

use crate::token::{ Token, Keyword };
use crate::node::Node;

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens: tokens.try_into().unwrap() }
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
                if let Some((Node::DataType { mut types }, tokens)) = self.parse_expr(n) {
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

    fn parse_stmt(&self, n: usize) -> Option<(Node, usize)> {
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
                                            return Some((Node::Define { 
                                                name, 
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
