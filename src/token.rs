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
    Numliteral {
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
