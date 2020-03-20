extern crate colored;

mod nss;

use nss::lexer::*;
use nss::source::Source;
use nss::parser::*;
use nss::compiler::*;

fn main() {
    let test = r#"
@variable = red
@cool_image = "/assets/cool_image.jpeg"

body, h1
  color: blue
  background: url(@cool_image)!


a, button
  color: @variable
    "#;

    let source = Source::from("<test>", test.lines().map(|x| x.into()).collect::<Vec<String>>());
    let lexer  = Lexer::default(test.chars().collect(), &source);

    let mut tokens = Vec::new();

    for token_r in lexer {
        if let Ok(token) = token_r {
            tokens.push(token)
        } else {
            return
        }
    }

    let mut parser = Parser::new(tokens, &source);

    match parser.parse() {
        Ok(ast) => {
            println!("{:#?}", ast);

            let mut compiler = Compiler::new();

            println!("---- output ----");
            println!("{}", compiler.compile(ast))
        }

        _ => return
    }
}