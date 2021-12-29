use clap::{App, Arg};
use compiler;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::env;
use std::fs;
use utils;

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
        .arg(
            Arg::with_name("dev-mode")
                .short("d")
                .long("dev")
                .takes_value(false)
                .help("Prevents work directory clean up."),
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

        if lexer.tree().len() == 0 {
            std::process::exit(1);
        }
        let mut global_scope: HashMap<String, compiler::parse::ParseTok> = HashMap::new();

        let mut parser = parse::Parser::new(lexer.tree(), file_content, global_scope);

        parser.init();
        let mut main_buffer: Vec<String> = vec![String::from("int main() {")];
        let def = &mut IndexMap::new();
        for tok in parser.tree() {
            let gen = compiler::generation::gen(
                compiler::generation::DescriptorToken {
                    token_real_type: None,
                    token: tok,
                },
                "_".to_string(),
                def,
            );
            main_buffer.push(gen);
        }
        main_buffer.push(String::from("return 0;\n}"));
        let mut defs: Vec<String> = def
            .values()
            .map(|item| -> String { item.def.clone() })
            .collect();
        defs.append(&mut main_buffer);
        defs.insert(0, "#include<memory>".to_string());
        let joined = defs.join("\n");
        utils::make_work(joined);
        if file.index_of("dev-mode").is_none() {
            utils::clean_work();
        }
    }
}
