use clap::{App, Arg};
use compiler;
use std::env;
use std::fs;

use compiler::*;
fn main() {
    let app = App::new("Something")
        .author("Ashtyn")
        .version(clap::crate_version!())
        .about("a stackish based programming language that is also functional.");

    let file = app
        .arg(
            Arg::with_name("file_name")
                .required(true)
                .help("input file filename"),
        )
        .get_matches();

    let f = fs::read_to_string(file.value_of("file_name").unwrap());

    if f.is_err() {
        println!(
            "Could not read file {}",
            file.value_of("file_name").unwrap()
        )
    } else {
        let file_content = f.unwrap();
        let mut lexer = Lexer::new(file_content.chars().collect());

        lexer.lex();

        let mut parser = parse::Parser::new(lexer.tree(), file_content);

        parser.parse();
    }
}
