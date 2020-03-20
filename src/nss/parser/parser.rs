use super::super::error::Response::Wrong;
use super::*;

use std::rc::Rc;

pub struct Parser<'a> {
    index: usize,
    tokens: Vec<Token>,
    source: &'a Source,
    indent_base: usize,
    indent: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, source: &'a Source) -> Self {
        Self {
            tokens,
            source,
            index: 0,
            indent_base: 0,
            indent: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, ()> {
        let mut ast = Vec::new();

        while self.remaining() > 0 {
            ast.push(self.parse_statement()?)
        }

        Ok(ast)
    }

    pub fn parse_statement(&mut self) -> Result<Statement, ()> {
        use self::TokenType::*;

        while self.current_type() == EOL && self.remaining() != 0 {
            self.next()?
        }

        let position = self.current_position();

        let statement = match self.current_type() {
            Identifier => {
                let mut names = Vec::new();

                names.push(self.eat()?);

                while self.current_lexeme() == "," {
                    self.next()?;

                    names.push(self.eat_type(&Identifier)?)
                }

                match self.current_lexeme().as_str() {
                    ":" => {
                        if names.len() > 1 {
                            return Err(
                                response!(
                                    Wrong("this makes no sense (many attributes)"),
                                    self.source.file,
                                    self.current_position()
                                )
                            )
                        }

                        self.next()?;

                        Statement::new(
                            StatementNode::Style(
                                names[0].clone(),
                                self.parse_expression()?
                            ),
                            self.span_from(position)
                        )
                    },

                    "\n" => {
                        self.eat_lexeme("\n")?;

                        let definitions = self.parse_body()?;

                        Statement::new(
                            StatementNode::Definition(names, definitions),
                            self.span_from(position)
                        )
                    },

                    c => return Err(
                        response!(
                            Wrong(format!("unexpected token: `{}`", c)),
                            self.source.file,
                            self.current_position()
                        )
                    )
                }
            },

            c => return Err(
                response!(
                    Wrong(format!("unexpected token: `{}`", c)),
                    self.source.file,
                    self.current_position()
                )
            )
        };

        if self.remaining() > 1 {
            self.new_line()?;
        }

        Ok(statement)
    }

    fn parse_body(&mut self) -> Result<Vec<Statement>, ()> {
        let backup_indent = self.indent;

        self.indent = self.get_indent();

        if self.indent_base == 0 {
            self.indent_base = self.indent
        } else {
            if self.indent % self.indent_base != 0 {
                return Err(
                    response!(
                        Wrong("inconsistent indentation is not cool"),
                        self.source.file,
                        self.current_position()
                    )
                )
            }
        }

        let mut accum = Vec::new();

        while !self.is_dedent() && self.remaining() > 0 {
            let statement = self.parse_statement()?;

            self.next_newline()?;

            accum.push(statement)
        }
        
        self.indent = backup_indent;

        Ok(accum)
    }

    pub fn parse_expression(&mut self) -> Result<Expression, ()> {
        let atom = self.parse_atom()?;

        if self.current_type() == TokenType::Operator {
            Ok(atom) // later
        } else {
            Ok(atom)
        }
    }

    pub fn parse_atom(&mut self) -> Result<Expression, ()> {
        use self::TokenType::*;

        if self.remaining() == 0 {
            Ok(
                Expression::new(
                    ExpressionNode::EOF,
                    self.current_position()
                )
            )
        } else {
            let token_type = self.current_type().clone();
            let position   = self.current_position();

            let expr = match token_type {
                Int => Expression::new(
                    ExpressionNode::Int(self.eat()?.parse::<i32>().unwrap()),
                    position
                ),

                Float => Expression::new(
                    ExpressionNode::Float(self.eat()?.parse::<f32>().unwrap()),
                    position
                ),

                Str => Expression::new(
                    ExpressionNode::Str(self.eat()?),
                    position
                ),

                Identifier => Expression::new(
                    ExpressionNode::Identifier(self.eat()?),
                    position
                ),

                ref tt => {
                    return Err(
                        response!(
                            Wrong(
                                format!("unexpected token `{}`", tt),
                            ),
                            self.source.file,
                            self.current_position()
                        )
                    )
                }
            };
        
            if self.remaining() > 0 {
                self.parse_postfix(expr)
            } else {
                Ok(expr)
            }
        }
    }

    fn parse_postfix(&mut self, expression: Expression) -> Result<Expression, ()> {
        let backup_index = self.index;

        match self.current_lexeme().as_str() {
            "(" => {
                self.next()?;
                self.next_newline()?;
                
                let mut args = Vec::new();

                while !["\n", ")"].contains(&self.current_lexeme().as_str()) {
                    args.push(self.parse_expression()?);

                    if !["\n", ")"].contains(&self.current_lexeme().as_str()) && self.remaining() > 0 {
                        self.eat_lexeme(",")?;
                        self.next_newline()?;
                    }
                }

                self.next_newline()?;
                self.eat_lexeme(")")?;

                let position = expression.pos.clone();

                let expr = Expression::new(
                    ExpressionNode::Call(
                        Rc::new(expression),
                        args,
                    ),
                    self.span_from(position),
                );

                return self.parse_postfix(expr)
            }

            "!" => {
                self.next()?;

                let position = expression.pos.clone();

                return Ok(
                    Expression::new(
                        ExpressionNode::Important(
                            Rc::new(expression)
                        ),
                        self.span_from(position)
                    )
                )
            }

            _ => ()
        }

        Ok(expression)
    }



    fn new_line(&mut self) -> Result<(), ()> {
        if self.remaining() > 0 {
            match self.current_lexeme().as_str() {
                "\n" => self.next(),
                _    => Err(
                    response!(
                        Wrong(
                            format!(
                                "expected new line, found: `{}`",
                                self.current_lexeme()
                            )
                        ),
                        self.source.file,
                        self.current_position()
                    )
                )
            }
        } else {
            Ok(())
        }
    }

    fn span_from(&self, position: Pos) -> Pos {
        let Pos(ref line, ref slice) = position;
        let Pos(_, ref slice2) = self.current_position();

        Pos(
            line.clone(),
            (
                slice.0,
                if slice2.1 < line.1.len() {
                    slice2.1
                } else {
                    line.1.len()
                }
            )
        )
    }

    fn remaining(&self) -> usize {
        self.tokens.len().saturating_sub(self.index)
    }

    fn next(&mut self) -> Result<(), ()> {
        if self.index <= self.tokens.len() {
            self.index += 1;

            Ok(())
        } else {
            Err(
                response!(
                    Wrong("nexting too far"),
                    self.source.file,
                    self.current_position()
                )
            )
        }
    }

    fn next_newline(&mut self) -> Result<(), ()> {
        while self.current_lexeme() == "\n" && self.remaining() > 0 {
            self.next()?
        }

        Ok(())
    }

    fn get_indent(&self) -> usize {
        self.current().slice.0 - 1
    }

    fn is_dedent(&self) -> bool {
        self.get_indent() < self.indent && self.current_lexeme() != "\n"
    }

    fn current(&self) -> Token {
        if self.index > self.tokens.len() - 1 {
            self.tokens[self.tokens.len() - 1].clone()
        } else {
            self.tokens[self.index].clone()
        }
    }

    fn current_lexeme(&self) -> String {
        self.current().lexeme.clone()
    }

    fn current_type(&self) -> TokenType {
        self.current().token_type.clone()
    }

    fn current_position(&self) -> Pos {
        let current = self.current();

        Pos(current.line.clone(), current.slice)
    }

    fn eat(&mut self) -> Result<String, ()> {
        let lexeme = self.current().lexeme;

        self.next()?;

        Ok(lexeme)
    }

    fn eat_lexeme(&mut self, lexeme: &str) -> Result<String, ()> {
        if self.current_lexeme() == lexeme {
            let lexeme = self.current().lexeme;
            self.next()?;

            Ok(lexeme)
        } else {
            Err(response!(
                Wrong(format!(
                    "expected `{}` but found `{}`",
                    lexeme,
                    self.current_lexeme()
                )),
                self.source.file,
                self.current_position()
            ))
        }
    }

    fn eat_type(&mut self, token_type: &TokenType) -> Result<String, ()> {
        if self.current_type() == *token_type {
            let lexeme = self.current().lexeme.clone();
            self.next()?;

            Ok(lexeme)
        } else {
            Err(response!(
                Wrong(format!(
                    "expected `{}` but found `{}`",
                    token_type,
                    self.current_type()
                )),
                self.source.file,
                self.current_position()
            ))
        }
    }

    fn expect_lexeme(&self, lexeme: &str) -> Result<(), ()> {
        if self.current_lexeme() == lexeme {
            Ok(())
        } else {
            Err(
                response!(
                    Wrong(
                        format!(
                            "expected `{}` but found `{}`",
                            lexeme,
                            self.current_lexeme()
                        )
                    ),
                    self.source.file
                )
            )
        }
    }
}