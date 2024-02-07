#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Exit,
    Define,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Int,
    Unit,
}

#[derive(Debug, Clone, Copy)]
pub enum NumLiteral {
    IntLiteral {
        value: i32
    },
}

impl std::fmt::Display for NumLiteral {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
            Self::IntLiteral { value } => {
                f.write_fmt(format_args!("{}", value))?; 
            },
       }
       Ok(())
   } 
}

#[derive(Debug, Clone)]
pub enum Token {
    Keyword {
        keyword: Keyword,
    },
    NumLiteral {
        literal: NumLiteral,
    },
    DataType {
        data_type: DataType,
    },
    SpecialArrow,
    TypeArrow,
    EndArrow,
    Identifier {
        name: String,
    },
    PipeArrow,
    PreserveArrow,
    PrependArrow,
}

impl DataType {
    pub fn data_types() -> Vec<Self> {
        vec![
            DataType::Int,
            DataType::Unit,
        ]
    }
}

impl NumLiteral {
    pub fn num_literals() -> Vec<Self> {
        vec![
            NumLiteral::IntLiteral { value: 0 },
        ]
    }
}

impl From<Token> for String {
    fn from(token: Token) -> Self {
        match token {
            Token::Keyword { keyword: kw } => match kw {
                Keyword::Exit => "exit".to_string(),
                Keyword::Define => "define".to_string(),
            },
            Token::NumLiteral { literal: lit } => match lit {
                NumLiteral::IntLiteral { value } => format!("{value}"),
            },
            Token::DataType { data_type: dt } => match dt {
                DataType::Int => "Int".to_string(),
                DataType::Unit => "()".to_string(),
            },
            Token::Identifier { name } => format!("{name}"),
            Token::SpecialArrow => ":>".to_string(),
            Token::EndArrow => ";>".to_string(),
            Token::TypeArrow => "->".to_string(),
            Token::PipeArrow => "=>".to_string(),
            Token::PreserveArrow => "|>".to_string(),
            Token::PrependArrow => "+>".to_string(),
        }
    }
}
