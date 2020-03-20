extern crate colored;

mod nss;

use nss::lexer::*;
use nss::source::Source;
use nss::parser::*;

fn main() {
    let test = r#"
-- KEBAB-CASE WORKING
body, h1
    color: a
    background: url(cool_image)!
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
        Ok(ref ast) => {
            println!("{:#?}", ast);
        }

        _ => return
    }
}