use std::collections::VecDeque;

use crate::token::{Token, Keyword, NumLiteral};

pub struct Lexer {
    source: VecDeque<u8>
}

impl Lexer {
    pub fn new(source: Vec<u8>) -> Self {
        Self { source: source.try_into().unwrap() }
    }

    fn peek(&self, n: usize) -> Option<u8> {
        if n >= self.source.len() {
            None
        } else {
            Some(self.source[n])
        }
    }

    fn consume(&mut self, n: usize) {
        for _ in 0..n {
            self.source.pop_front();
        }
    }

    fn parse_keyword(&self, word: &str) -> bool {
        fn parse_keyword_helper(source: &VecDeque<u8>, word: &[u8], n: usize) -> bool {
            if n == word.len() {
                return true;
            }
            if source[n] == word[n] {
                return parse_keyword_helper(source, word, n + 1);
            }

            false
        }

        if self.peek(word.len()-1).is_some() {
            parse_keyword_helper(&self.source, word.as_bytes(), 0)
        } else { 
            false 
        }
    }

    fn parse_literal(&self) -> Option<(NumLiteral, usize)> {
        let mut number = 0;
        let mut sgn = 1;
        let mut i = 0;

        if self.peek(0) == Some(b'-') {
            sgn = -1;
            i = 1;
        }

        if let Some(byte) = self.peek(i) {
            if !byte.is_ascii_digit() {
                return None;
            }
        }

        while let Some(byte) = self.peek(i) {
            if !byte.is_ascii_digit() {
                break;
            }
            number *= 10;
            number += (byte - b'0') as i32;
            i += 1;
        }
        
        Some((NumLiteral::IntLiteral { value: number * sgn }, i))
    }

    fn token(&mut self) -> Option<Token> {
        while let Some(byte) = self.peek(0) {
            if !byte.is_ascii_whitespace() {
                break;
            }
            self.consume(1);
        }

        if self.peek(0).is_none() {
            return None;
        }

        if self.parse_keyword("exit") {
            self.consume(4);
            return Some(Token::Keyword { keyword: Keyword::Exit });
        }

        if let Some((literal, bytes)) = self.parse_literal() {
            self.consume(bytes);
            return Some(Token::Numliteral { literal });
        }

        None
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.token() {
            tokens.push(token);
        }
        tokens
    }
}
