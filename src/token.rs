#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keyword {
    Define,
    If,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DataType {
    Int,
    Unit,
    String,
    Bool,
    Template(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    IntLiteral(i32),
    StringLiteral(String),
    BoolLiteral(bool),
}

impl std::fmt::Display for Literal {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
            Self::IntLiteral(value) => {
                f.write_fmt(format_args!("{}", value))?; 
            },
            Self::StringLiteral(value) => {
                f.write_fmt(format_args!("\"{}\"", value))?;
            },
            Self::BoolLiteral(value) => {
                f.write_fmt(format_args!("{}", value))?;
            }
       }
       Ok(())
   } 
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Literal(Literal),
    DataType(DataType),
    SpecialArrow,
    TypeArrow,
    EndArrow,
    Identifier(String),
    PipeArrow,
    PreserveArrow,
    PrependArrow,
}

impl Keyword {
    fn keywords() -> Vec<Self> {
        vec![
            Keyword::Define,
            Keyword::If,
        ]
    }
    pub fn src_repr(&self) -> String {
        match self {
            Keyword::Define => "define",
            Keyword::If => "if",
        }.to_string()
    }
}

impl DataType {
    fn data_types() -> Vec<Self> {
        vec![
            DataType::Int,
            DataType::Unit,
            DataType::String,
            DataType::Bool,
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

    pub fn keywords() -> Vec<Keyword> {
        Keyword::keywords()
    }
    
    pub fn data_types() -> Vec<DataType> {
        DataType::data_types()
    }

    pub fn arrow_src_repr(&self) -> String {
        match self {
            Token::PrependArrow => "+>",
            Token::TypeArrow => "->",
            Token::PipeArrow => "=>",
            Token::SpecialArrow => ":>",
            Token::EndArrow => ";>",
            Token::PreserveArrow => "|>",
            _ => ""
        }.to_string()
    }
}

impl DataType {
    pub fn c_repr(&self) -> String {
        match self {
            DataType::Unit => "Unit",
            DataType::Int => "Int",
            DataType::String => "String",
            DataType::Bool => "Bool",
            _ => ""
        }.to_string()
    }
    pub fn src_repr(&self) -> String {
        match self {
            DataType::Unit => "()",
            DataType::Int => "Int",
            DataType::String => "String",
            DataType::Bool => "Bool",
            _ => ""
        }.to_string()
    }

    fn predecessors(&self) -> Vec<DataType> {
        match self {
            _ => vec![]
        }
    }

    pub fn inherits(&self, other: &Self) -> bool {
       if other.predecessors().contains(self) {
            return true;
       }

       if let DataType::Template(_) = other {
            return true;
       }

       false
    }
}

impl Literal {
    pub fn c_type_repr(&self) -> String {
        match self {
            Literal::IntLiteral(_) => "Int",
            Literal::StringLiteral(_) => "String",
            Literal::BoolLiteral(_) => "Bool",
        }.to_string()
    }
}

impl From<Literal> for DataType {
    fn from(lit: Literal) -> Self {
        match lit {
            Literal::IntLiteral(_) => DataType::Int,
            Literal::StringLiteral(_) => DataType::String,
            Literal::BoolLiteral(_) => DataType::Bool,
        }
    }
}
