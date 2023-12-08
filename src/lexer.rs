use std::collections::VecDeque;

use crate::token::{Token, Keyword, NumLiteral, DataType};

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

    fn parse_word(&self, word: &str) -> bool {
        fn parse_keyword_helper(source: &VecDeque<u8>, word: &[u8], n: usize) -> bool {
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
        
        Some((NumLiteral::IntLiteral { value: number }, i))
    }

    fn parse_identifier(&self) -> Option<(String, usize)> {
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
        
        unsafe {
            Some((String::from_utf8_unchecked(identifier), i))
        }
    
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

        if self.parse_word("exit") {
            self.consume(4);
            return Some(Token::Keyword { keyword: Keyword::Exit });
        }

        if self.parse_word("define") {
            self.consume(6);
            return Some(Token::Keyword { keyword: Keyword::Define });
        }

        if self.parse_word("Int") {
            self.consume(3);
            return Some(Token::DataType { data_type: DataType::Int });
        }

        if self.parse_word("()") {
            self.consume(2);
            return Some(Token::DataType { data_type: DataType::Unit });
        }

        if self.parse_word("->") {
            self.consume(2);
            return Some(Token::TypeArrow);
        }

        if self.parse_word(":>") {
            self.consume(2);
            return Some(Token::SpecialArrow);
        }

        if self.parse_word(";>") {
            self.consume(2);
            return Some(Token::EndArrow);
        }

        if self.parse_word("=>") {
            self.consume(2);
            return Some(Token::PipeArrow);
        }

        if let Some((literal, bytes)) = self.parse_literal() {
            self.consume(bytes);
            return Some(Token::Numliteral { literal });
        }

        if let Some((name, bytes)) = self.parse_identifier() {
            self.consume(bytes);
            return Some(Token::Identifier { name });
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
