extern crate colored;

mod nss;

use nss::lexer::*;
use nss::source::Source;
use nss::parser::*;
use nss::compiler::*;

use std::fs::{ File, metadata, read_dir, write, remove_file };
use std::{env, io::prelude::Read};
use env::args;
use std::path::Path;

use colored::Colorize;

fn new_path(path: &String) -> String {
    let mut paths = path.split("/").collect::<Vec<&str>>();
    let file_name = paths.last().unwrap();
    let new_file_name = file_name.replace(".nss", ".css");

    paths.reverse();

    let mut new_path = paths[1..].to_vec();
    new_path.reverse();

    format!("{}/{}", new_path.join("/"), new_file_name)
}

fn is_nss(path: &String) -> bool {
    let paths = path.split("/").collect::<Vec<&str>>();
    let file_name = paths.last().unwrap();

    file_name.split(".").collect::<Vec<&str>>().last().unwrap() == &"nss"
}

fn compile_path(path: &String) {
    let meta = metadata(&path).expect("Failed to get metadata.");

    if meta.is_dir() {
        let dir = read_dir(&path).expect("Failed to read directory.");

        for path in dir {
            let path = path.unwrap().path().display().to_string();
            compile_path(&path)
        }
    } else {
        if is_nss(path) {
            match run(&path) {
                Some(content) => {
                    let status = format!("{} {}", "Compiled".green().bold(), path);
                    write(new_path(&path), content).expect("Failed to write file.");
                    println!("{}", status)
                },
    
                None => ()
            }
        }
    }
}

fn clean_path(path: &String) {
    let meta = metadata(&path).expect("Failed to get metadata.");

    if meta.is_dir() {
        let dir = read_dir(&path).expect("Failed to read directory.");

        for path in dir {
            let path = &path.unwrap().path().display().to_string();
            if is_nss(path) {
                clean_path(path)
            }
        }
    } else {
        if is_nss(path) {
            let path = new_path(&path);
            if Path::new(&path).exists() {
                let status = format!("{} {}", "Cleaned".magenta().bold(), path);
                remove_file(path).expect("Failed to remove file.");
                println!("{}", status)
            }
        }
    }
}

fn run(path: &String) -> Option<String> {
    let mut source = File::open(path.as_str()).unwrap();
    let mut content = String::new();

    source.read_to_string(&mut content).unwrap();

    let source = Source::from(&path, content.lines().map(|x| x.into()).collect::<Vec<String>>());
    let lexer  = Lexer::default(content.chars().collect(), &source);

    let mut tokens = Vec::new();

    for token_r in lexer {
        if let Ok(token) = token_r {
            tokens.push(token)
        } else {
            return None
        }
    }

    let mut parser = Parser::new(tokens, &source);

    match parser.parse() {
        Ok(ast) => {
            let mut compiler = Compiler::new();
            let result = compiler.compile(ast);

            Some(result)
        }

        _ => None
    }
}

const HELP: &'static str = r#"
(N)IELS (S)TYLE (S)HEETS
========================

- nss <path>          # compile files in folder
- nss <file>...       # compile one or more files
- nss clean <path>... # remove compiled css files in one or more paths
"#;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        if args[1] == "clean" {
            if args.len() > 2 {
                for arg in args[2..].iter() {
                    clean_path(arg)
                }
            } else {

            }
        } else {
            for arg in args[1..].iter() {
                compile_path(arg)
            }
        }
    } else {
        println!("{}", HELP)
    }
}