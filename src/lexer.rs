use std::collections::VecDeque;

use crate::token::{
    DataType, Keyword, Literal, Token 
};

pub fn tokenize(source: Vec<u8>) -> Option<Vec<Token>> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    while let Some(token) = lexer.tokenize() {
        tokens.push(token);
    }
    if lexer.source.is_empty() {
        Some(tokens)
    } else {
        None
    }
}

struct Lexer {
    source: VecDeque<u8>,
}

impl Lexer {
    fn new(source: Vec<u8>) -> Self {
        Self { 
            source: source.into(),
        }
    }

    fn tokenize(&mut self) -> Option<Token> {
        self.consume_whitespace();
        self.check_if_some()?;
        if let Some(kw) = self.tokenize_keyword() {
            return Some(Token::Keyword(kw));
        }
        if let Some(dt) = self.tokenize_datatype() {
            return Some(Token::DataType(dt));
        }
        if let Some(arr) = self.tokenize_arrow() {
            return Some(arr);
        }
        if let Some(lit) = self.tokenize_literal() {
            return Some(Token::Literal(lit));
        }
        if let Some(ident) = self.tokenize_identifier(false) {
            return Some(Token::Identifier(ident));
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

    fn check_if_some(&self) -> Option<()> {
        self.peek(0)?;
        Some(())
    }

    fn tokenize_keyword(&mut self) -> Option<Keyword> {
        for keyword in Token::keywords() {
            if self.try_tokenize(keyword.src_repr().as_str()) {
                return Some(keyword);
            }
        }
        None
    }

    fn tokenize_datatype(&mut self) -> Option<DataType> {
        for data_type in Token::data_types() {
            if self.try_tokenize(data_type.src_repr().as_str()) {
                return Some(data_type);
            }
        }
        if let Some(ident) = self.tokenize_identifier(true) {
            return Some(DataType::Template(ident));
        }
        None
    }

    fn tokenize_arrow(&mut self) -> Option<Token> {
        for arr in Token::arrows() {
            if self.try_tokenize(arr.arrow_src_repr().as_str()) {
                return Some(arr);
            }
        }
        None
    }

    fn try_tokenize(&mut self, token_str: &str) -> bool {
        if self.peek(token_str.len()-1).is_none() {
            return false;
        }

        if Self::try_tokenize_helper(&self.source, token_str.as_bytes(), 0) {
            self.consume(token_str.len());
            return true;
        }

        false
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

    fn tokenize_literal(&mut self) -> Option<Literal> {
        if let Some(lit) = self.tokenize_int_literal() {
            return Some(lit);
        }
    
        if let Some(lit) = self.tokenize_string_literal() {
            return Some(lit);
        }

        if let Some(lit) = self.tokenize_bool_literal() {
            return Some(lit);
        }

        None
    }

    fn tokenize_int_literal(&mut self) -> Option<Literal> {
        let mut number = 0;
        let mut i = 0;
        let neg = self.peek(0)? == '-' as u8;
        if neg {
            i += 1;
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
        self.consume(i);

        Some(Literal::IntLiteral(number * if neg {-1} else {1}))
    }

    fn tokenize_string_literal(&mut self) -> Option<Literal> {
        if self.peek(0) != Some(b'"') {
            return None;
        }
        self.consume(1);
        let mut i = 0;
        while let Some(byte) = self.peek(i) {
            if byte == b'"' {
                break;
            }
            i += 1;
        }
        if self.peek(i).is_none() {
            return None;
        }
        let chars: Vec<char> = (0..i).map(|x: usize|{self.peek(x).unwrap().into()}).collect();
        let s = String::from_iter(chars);
        self.consume(i+1);
        Some(Literal::StringLiteral(s))
    }

    fn tokenize_bool_literal(&mut self) -> Option<Literal> {
        if self.try_tokenize("true") {
            return Some(Literal::BoolLiteral(true));
        }

        if self.try_tokenize("false") {
            return Some(Literal::BoolLiteral(false));
        }

        None
    }

    fn tokenize_identifier(&mut self, cap_fst: bool) -> Option<String> {
        let mut identifier = Vec::new();
        let mut i = 0;

        if let Some(byte) = self.peek(i) {
            if !byte.is_ascii_alphabetic() {
                return None;
            }
            if cap_fst && byte.is_ascii_lowercase() {
                return None;
            }
            if !cap_fst && byte.is_ascii_uppercase() {
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
        
        Some(String::from_utf8(identifier).unwrap())
    }

    fn peek(&self, n: usize) -> Option<u8> {
        self.source.get(n).cloned()
    }

    fn consume(&mut self, n: usize) {
        self.source.drain(0..n);
    }
}
