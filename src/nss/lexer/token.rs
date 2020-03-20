use colored::Colorize;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Identifier,
    Symbol,
    Operator,
    Str,
    Float,
    Int,
    Whitespace,
    EOL,
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TokenType::*;

        match *self {
            Identifier => write!(f, "identifier"),
            Symbol     => write!(f, "symbol"),
            Operator   => write!(f, "operator"),
            Str        => write!(f, "string"),
            Float      => write!(f, "float"),
            Int        => write!(f, "int"),
            Whitespace => write!(f, "whitespace"),
            EOL        => write!(f, "new-line"),
            EOF        => write!(f, "end of file"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pos(pub (usize, String), pub (usize, usize));

impl Pos {
    pub fn get_lexeme(&self) -> String {
        (self.0).1[(self.1).0 - if (self.1).0 > 0 { 1 } else { 0 } .. (self.1).1].to_string()
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let linepad = format!("{:5} |", " ").blue().bold();
        let line_n  = format!("{:5} | ", (self.0).0).blue().bold();

        let mut mark = (self.0).1[(self.1).0.saturating_sub(1) .. (self.1).1].to_string();

        if mark.split_whitespace().count() == 0 {
            mark = format!("{:->count$}", "->".red().bold(), count = mark.len())
        } else {
            mark = format!("{}", mark.red().bold())
        }

        let mut arrows = format!("{: <count$}", " ", count = (self.1).0);

        for _ in 0 .. (self.1).1 - (self.1).0 + 1 {
            arrows.push('^')
        }

        write!(
            f,
            "\n{}\n{}{}{}{}\n{}{}",
            linepad,
            line_n,
            &(self.0).1[..(self.1).0.saturating_sub(1)],
            mark,
            &(self.0).1[(self.1).1 ..],
            linepad,
            arrows.red().bold(),
        )
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: (usize, String),
    pub slice: (usize, usize),
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, line: (usize, String), slice: (usize, usize), lexeme: &str) -> Self {
        Self {
            token_type,
            line,
            slice,
            lexeme: lexeme.to_string()
        }
    } 
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Pos(
            (self.line.0, self.line.1.clone()),
            (self.slice.0, self.slice.1)
        ))
    }
}