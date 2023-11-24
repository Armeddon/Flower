#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    Exit,
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

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Keyword {
        keyword: Keyword,
    },
    Numliteral {
        literal: NumLiteral,
    },
}
