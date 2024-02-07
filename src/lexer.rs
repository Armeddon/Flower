use std::collections::VecDeque;

use crate::token::{
    Token, 
    NumLiteral, 
};

pub fn tokenize(source: Vec<u8>) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.tokenize() {
        tokens.push(token);
    }
    tokens
}

struct Lexer {
    source: VecDeque<u8>
}

impl Lexer {
    fn new(source: Vec<u8>) -> Self {
        Self { source: source.try_into().unwrap() }
    }

    fn tokenize(&mut self) -> Option<Token> {
        self.consume_whitespace();
        self.check_if_none()?;
        if let Some(kw) = self.tokenize_keyword() {
            return Some(kw);
        }
        if let Some(dt) = self.tokenize_datatype() {
            return Some(dt);
        }
        if let Some(arr) = self.tokenize_arrow() {
            return Some(arr);
        }
        if let Some(lit) = self.tokenize_literal() {
            return Some(lit);
        }
        if let Some(ident) = self.tokenize_identifier() {
            return Some(ident);
        }
        None
    }

    fn consume_whitespace(&mut self) -> () {
        while let Some(byte) = self.peek(0) {
            if !byte.is_ascii_whitespace() {
                break;
            }
            self.consume(1);
        }
    }

    fn check_if_none(&self) -> Option<()> {
        self.peek(0)?;
        Some(())
    }

    fn tokenize_keyword(&mut self) -> Option<Token> {
        for keyword in Token::keywords() {
            if self.try_tokenize(&keyword).is_none() {
                continue
            }
            return Some(keyword);
        }
        None
    }

    fn tokenize_datatype(&mut self) -> Option<Token> {
        for data_type in Token::data_types() {
            if self.try_tokenize(&data_type).is_none() {
                continue;
            }
            return Some(data_type);
        }
        None
    }

    fn tokenize_arrow(&mut self) -> Option<Token> {
        for arr in Token::arrows() {
            if self.try_tokenize(&arr).is_none() {
                continue;
            }
            return Some(arr);
        }
        None
    }

    fn try_tokenize(&mut self, token: &Token) -> Option<Token> {
        let token_str: String = token.clone().into();

        if self.peek(token_str.len()-1).is_none() {
            return None;
        }

        if Self::try_tokenize_helper(&self.source, token_str.as_bytes(), 0) {
            self.consume(token_str.len());
            return Some(token.clone());
        }

        None
    }

    fn try_tokenize_helper(source: &VecDeque<u8>, word: &[u8], n: usize) -> bool {
       if n == word.len() {
            if let Some(byte) = source.get(n - 1) {
                if !byte.is_ascii_alphanumeric() {
                    return true;
                }
            }
            if let Some(byte) = source.get(n) {
                return !byte.is_ascii_alphanumeric();
            }
            return true;
        }
        if source[n] == word[n] {
            return Self::try_tokenize_helper(source, word, n + 1);
        }
        false
    }

    fn tokenize_literal(&mut self) -> Option<Token> {
        let mut number = 0;
        let mut i = 0;

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
        self.consume(i);

        Some(Token::NumLiteral {
            literal: NumLiteral::IntLiteral {
                value: number 
            }
        })
    }

    fn tokenize_identifier(&mut self) -> Option<Token> {
        let mut identifier = Vec::new();
        let mut i = 0;

        if let Some(byte) = self.peek(i) {
            if !byte.is_ascii_alphabetic() {
                return None;
            }
        }
        while let Some(byte) = self.peek(i) {
            if !byte.is_ascii_alphanumeric() {
                break;
            }
            identifier.push(byte);
            i += 1;
        }
        self.consume(i);
        
        Some(
            Token::Identifier { 
                name:  unsafe{String::from_utf8_unchecked(identifier)},
            }
        )
    }

    fn peek(&self, n: usize) -> Option<u8> {
        self.source.get(n).cloned()
    }

    fn consume(&mut self, n: usize) {
        self.source.drain(0..n);
    }
}
