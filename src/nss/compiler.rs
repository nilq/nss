use super::parser::*;

use std::collections::HashMap;

pub struct Compiler {
    variables: HashMap<String, Expression>
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new()
        }
    }

    pub fn compile(&mut self, ast: Vec<Statement>) -> String {
        let mut output = String::new();
        
        for s in ast.iter() {
            let out = self.compile_statement(&s);
            output.push_str(out.as_str())
        }

        output
    }

    pub fn compile_statement(&mut self, statement: &Statement) -> String {
        use self::StatementNode::*;
        
        match statement.node {
            Definition(ref names, ref styles) => {
                let mut result = String::new();

                for (i, name) in names.iter().enumerate() {
                    result.push_str(
                        &format!("{}", name)
                    );

                    if i != names.len() - 1 {
                        result.push_str(", ");
                    }
                }

                result.push_str(" {\n");

                for style in styles.iter() {
                    Self::push_line(
                        &mut result,
                        &self.compile_statement(style)
                    )
                }

                result.push_str("}\n\n");

                result
            },

            Style(ref name, ref expr) => format!(
                "{}: {};",
                name,
                self.compile_expression(&expr)
            ),

            Var(ref name, ref expr) => {
                self.variables.insert(name.to_owned(), expr.clone());
                String::new()
            }

            _ => String::new(),
        }
    }

    fn compile_expression(&self, expression: &Expression) -> String {
        use self::ExpressionNode::*;

        match expression.node {
            Deref(ref n) => self.compile_expression(self.variables.get(n).unwrap()),
            Identifier(ref n) => format!("{}", n),
            Str(ref n) => format!("\"{}\"", n),
            Call(ref n, ref args) => {
                let mut out = format!("{}(", self.compile_expression(n));

                for (i, arg) in args.iter().enumerate() {
                    out.push_str(&self.compile_expression(&arg));

                    if i != args.len() - 1 {
                        out.push_str(", ")
                    }
                }

                out.push_str(")");

                out
            },
            Important(ref n) => format!(
                "{} !important",
                self.compile_expression(n),
            ),

            _ => String::new()
        }
    }

    fn make_line(value: &str) -> String {
        let mut out = String::new();

        for line in value.lines() {
            out.push_str("  ");
            out.push_str(&line);
            out.push('\n')
        }

        out
    }

    fn push_line(target: &mut String, value: &str) {
        target.push_str(&Self::make_line(&value))
    }
}