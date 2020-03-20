use std::rc::Rc;
use std::fmt;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum StatementNode {
    Expression(Expression),
    Definition(Vec<String>, Vec<Statement>),
    Style(String, Expression),
    Var(String, Expression)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub node: StatementNode,
    pub pos: Pos,
}

impl Statement {
    pub fn new(node: StatementNode, pos: Pos) -> Self {
        Self { node, pos }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    Int(i32),
    Float(f32),
    Str(String),
    Identifier(String),
    Color(String),
    Call(Rc<Expression>, Vec<Expression>),
    Binary(Rc<Expression>, Operator, Rc<Expression>),
    Important(Rc<Expression>),
    Deref(String),
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression { // bruh
    pub node: ExpressionNode,
    pub pos: Pos,
}

impl Expression {
    pub fn new(node: ExpressionNode, pos: Pos) -> Self {
        Self { node, pos }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add, Sub, Mul, Div, Pow
}

impl Operator {
    pub fn from_str(operator: &str) -> Option<(Operator, u8)> {
        use self::Operator::*;

        let precedence = match operator {
            "+" => (Add, 0),
            "-" => (Sub, 0),
            "*" => (Mul, 1),
            "/" => (Div, 1),
            "^" => (Pow, 2),
            _   => return None
        };

        Some(precedence)
    }

    pub fn as_str(&self) -> &str {
        use self::Operator::*;

        match *self {
            Add => "+",
            Sub => "-",
            Mul => "*",
            Div => "/",
            Pow => "^"
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}