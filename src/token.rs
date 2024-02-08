#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    Define,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Int,
    Unit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NumLiteral {
    IntLiteral (i32),
}

impl std::fmt::Display for NumLiteral {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
            Self::IntLiteral (value) => {
                f.write_fmt(format_args!("{}", value))?; 
            },
       }
       Ok(())
   } 
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword (Keyword),
    NumLiteral (NumLiteral),
    DataType (DataType),
    SpecialArrow,
    TypeArrow,
    EndArrow,
    Identifier (String),
    PipeArrow,
    PreserveArrow,
    PrependArrow,
}

impl Keyword {
    fn keywords() -> Vec<Self> {
        vec![
            Keyword::Define,
        ]
    }
}

impl DataType {
    fn data_types() -> Vec<Self> {
        vec![
            DataType::Int,
            DataType::Unit,
        ]
    }
}

impl Token {
    pub fn arrows() -> Vec<Token> {
        vec![
            Token::SpecialArrow,
            Token::EndArrow,
            Token::TypeArrow,
            Token::PipeArrow,
            Token::PreserveArrow,
            Token::PrependArrow,
        ]
    }

    pub fn keywords() -> Vec<Token> {
        Keyword::keywords()
            .into_iter()
            .map(|keyword: Keyword| Token::Keyword (keyword))
            .collect()
    }
    
    pub fn data_types() -> Vec<Token> {
        DataType::data_types()
            .into_iter()
            .map(|data_type: DataType| Token::DataType (data_type))
            .collect()
    }
}

impl From<Token> for String {
    fn from(token: Token) -> Self {
        match token {
            Token::Keyword (kw) => match kw {
                Keyword::Define => "define".to_string(),
            },
            Token::NumLiteral (lit) => match lit {
                NumLiteral::IntLiteral (value) => format!("{value}"),
            },
            Token::DataType (dt) => match dt {
                DataType::Int => "Int".to_string(),
                DataType::Unit => "()".to_string(),
            },
            Token::Identifier (name) => format!("{name}"),
            Token::SpecialArrow => ":>".to_string(),
            Token::EndArrow => ";>".to_string(),
            Token::TypeArrow => "->".to_string(),
            Token::PipeArrow => "=>".to_string(),
            Token::PreserveArrow => "|>".to_string(),
            Token::PrependArrow => "+>".to_string(),
        }
    }
}

impl From<DataType> for String {
    fn from(dt: DataType) -> Self {
        Token::DataType(dt).into()
    }
}

impl DataType {
    pub fn c_string(&self) -> String {
        match self {
            DataType::Unit => "void".to_string(),
            DataType::Int => "int".to_string(),
        }
    }
}

impl From<NumLiteral> for DataType {
    fn from(nl: NumLiteral) -> Self {
        match nl {
            NumLiteral::IntLiteral(_) => DataType::Int,
        }
    }
}
