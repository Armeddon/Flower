use std::collections::{VecDeque, HashMap};

use crate::token::{ Token, Keyword, DataType };
use crate::node::{ Define, Funcall, Expr, Node, Pipe };

pub fn parse(tokens: Vec<Token>) -> Option<Vec<Node>> {
    let mut parser = Parser::new(tokens);
    let mut stmts = Vec::new();
    while let Some(stmt) = parser.try_parse_stmt() {
        stmts.push(stmt);
    }
    if !parser.tokens.is_empty() {
        None
    } else {
        Some(stmts)
    }
}

fn std_functions() -> HashMap<String, Vec<DataType>> {
    let mut map = HashMap::new();
    map.insert("readInt".to_string(), Vec::from([DataType::Int]));
    map.insert("readString".to_string(), Vec::from([
         DataType::Int,
         DataType::String
    ]));
    map.insert("println".to_string(), Vec::from([
        DataType::Template("T".to_string()),
        DataType::Unit,
    ]));
    map.insert("add".to_string(), Vec::from([
        DataType::Int,
        DataType::Int,
        DataType::Int,
    ]));
    map.insert("id".to_string(), Vec::from([
       DataType::Template("T".to_string()),
       DataType::Template("T".to_string()),
    ]));
    map.insert("eq".to_string(), Vec::from([
        DataType::Template("T".to_string()),
        DataType::Template("T".to_string()),
        DataType::Bool,
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

    fn try_parse_stmt(&mut self) -> Option<Node> {
        if let Some(kw) = self.try_parse_keyword() {
            return Some(kw);
        }
        if let Some(fc) = self.try_parse_funcall() {
            return Some(Node::Funcall(fc));
        }
        if let Some(expr) = self.try_parse_expr() {
            return Some(Node::Return(Box::from(expr)));
        }
        None
    }

    fn try_parse_expr(&mut self) -> Option<Expr> {
        if let Some(lit) = self.try_parse_literal() {
            return Some(lit);
        }
        None
    }

    fn try_parse_literal(&mut self) -> Option<Expr> {
        if let Some(Token::Literal(literal)) = self.peek(0) {
            self.consume(1);
            return Some(Expr::Literal(literal));
        }
        None
    }

    fn try_parse_funcall(&mut self) -> Option<Funcall> {
        let Token::Identifier(name) = self.peek(0)? else {
            return None;
        };
        self.consume(1);
        let mut in_place_params = Vec::new();
        while let Some(Token::Literal(literal)) = self.peek(0) {
            in_place_params.push(literal);
            self.consume(1);
        }
        let pipe_arrow = match self.peek(0) {
            Some(Token::PipeArrow) => Some(Token::PipeArrow),
            Some(Token::PrependArrow) => Some(Token::PrependArrow),
            Some(Token::PreserveArrow) => Some(Token::PreserveArrow),
            _ => None,
        };
        if pipe_arrow.is_some() {
            self.consume(1);
        }
        let pipe_funcall = if pipe_arrow.is_some() { 
            self.try_parse_funcall()
        } else {None};
        let pipe = pipe_funcall.map(|funcall: Funcall| {
            Box::from(funcall)
        });
        Some(Funcall{
            this_func_type: self.this_function.clone(),
            func_name: name.clone(),
            func_type: if let Some(types) = self.functions.get(&name) {
                types.clone()
            } else { return None; },
            in_place_params,
            pipe,
            pipe_type: {
                match pipe_arrow {
                    None => None,
                    Some(token) => match token {
                        Token::PipeArrow => Some(Pipe::Normal),
                        Token::PreserveArrow => Some(Pipe::Preserve),
                        Token::PrependArrow => Some(Pipe::Prepend),
                        _ => None,
                    }
                }
            },
        })
    }

    fn try_parse_keyword(&mut self) -> Option<Node> {
        if self.peek(0).is_none() {
            return None;
        }
        let Token::Keyword(keyword) = self.peek(0).unwrap()
        else {
            return None;
        };
        self.consume(1);
        match keyword {
            Keyword::Define => {
                self.try_parse_define()
                    .map(|define: Define|{Node::Define(define)})
            },
        }
    }

    fn try_parse_define(&mut self) -> Option<Define> {
        let Token::Identifier(name) = self.peek(0)? else {
            return None;
        };
        self.consume(1);
        if self.peek(0)? != Token::SpecialArrow {
            return None;
        }
        self.consume(1);
        let types = self.try_parse_data_type()?;
        if self.peek(0)? != Token::SpecialArrow {
            return None;
        }
        self.consume(1);
        let mut stmts = Vec::new();
        self.this_function = types.clone();
        loop {
            if self.peek(0)? == Token::EndArrow {
                self.consume(1);
                break;
            }
            let Some(stmt) = self.try_parse_stmt() else {
                return None;
            };
            stmts.push(stmt);
        }
        self.functions.insert(name.clone(), types.clone().try_into().unwrap());
        Some(Define{
            func_name: name,
            func_type: types.try_into().unwrap(),
            body: stmts,
        })
    }

    fn try_parse_data_type(&mut self) -> Option<Vec<DataType>> { 
        let Token::DataType(data_type) = self.peek(0)? else
        {
            return None;
        };

        self.consume(1);

        if self.peek(0).is_none() {
            return Some(vec![data_type]);
        }
        if self.peek(0).unwrap() != Token::TypeArrow {
            return Some(vec![data_type]);
        }
        self.consume(1);
        if let Some(types) = self.try_parse_data_type() {
            return Some({
                let mut types_dequeue: VecDeque<_> = types.try_into().unwrap();
                types_dequeue.push_front(data_type);
                types_dequeue.into()
            });
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

    fn consume(&mut self, n: usize) {
        for _ in 0..n {
            self.tokens.pop_front();
        }
    }
}
