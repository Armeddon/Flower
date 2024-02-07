use std::collections::VecDeque;

use crate::token::{Token, Keyword, NumLiteral, DataType};

struct Lexer {
    source: VecDeque<u8>
}

pub fn tokenize(source: Vec<u8>) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.tokenize() {
        tokens.push(token);
    }
    tokens
}

impl Lexer {
    fn new(source: Vec<u8>) -> Self {
        Self { source: source.try_into().unwrap() }
    }

    fn tokenize(&mut self) -> Option<Token> {
        self.tokenize_whitespace();

        self.tokenize_none()?;

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

    fn tokenize_whitespace(&mut self) -> () {
        while let Some(byte) = self.peek(0) {
            if !byte.is_ascii_whitespace() {
                break;
            }
            self.consume(1);
        }
    }

    fn tokenize_none(&self) -> Option<()> {
        self.peek(0)?;
        Some(())
    }

    fn tokenize_keyword(&mut self) -> Option<Token> {
        if self.tokenize_word("exit") {
            self.consume(4);
            return Some(Token::Keyword { keyword: Keyword::Exit });
        }

        if self.tokenize_word("define") {
            self.consume(6);
            return Some(Token::Keyword { keyword: Keyword::Define });
        }
        None
    }

    fn tokenize_datatype(&mut self) -> Option<Token> {
        if self.tokenize_word("Int") {
            self.consume(3);
            return Some(Token::DataType { data_type: DataType::Int });
        }

        if self.tokenize_word("()") {
            self.consume(2);
            return Some(Token::DataType { data_type: DataType::Unit });
        }
        None
    }

    fn tokenize_arrow(&mut self) -> Option<Token> {
       if self.tokenize_word("->") {
            self.consume(2);
            return Some(Token::TypeArrow);
        }

        if self.tokenize_word(":>") {
            self.consume(2);
            return Some(Token::SpecialArrow);
        }

        if self.tokenize_word(";>") {
            self.consume(2);
            return Some(Token::EndArrow);
        }

        if self.tokenize_word("=>") {
            self.consume(2);
            return Some(Token::PipeArrow);
        }

        if self.tokenize_word("|>") {
            self.consume(2);
            return Some(Token::PreserveArrow);
        }

        if self.tokenize_word("+>") {
            self.consume(2);
            return Some(Token::PrependArrow);
        }
        None
    }

    fn tokenize_word(&self, word: &str) -> bool {
        fn tokenize_keyword_helper(source: &VecDeque<u8>, word: &[u8], n: usize) -> bool {
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
                return tokenize_keyword_helper(source, word, n + 1);
            }

            false
        }

        if self.peek(word.len()-1).is_some() {
            tokenize_keyword_helper(&self.source, word.as_bytes(), 0)
        } else { 
            false 
        }
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
        
        unsafe {
            Some(
                Token::Identifier { 
                    name:  String::from_utf8_unchecked(identifier),
                }
            )
        }
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
}
