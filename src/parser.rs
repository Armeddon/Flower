use std::collections::VecDeque;

use crate::token::{ Token, Keyword, NumLiteral };
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
            Some(self.tokens[n])
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
