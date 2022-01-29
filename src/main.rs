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
        .about("A functional programming language.");

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
        .arg(
            Arg::with_name("gen")
                .short("g")
                .long("gen")
                .takes_value(false)
                .help("Stops compiler at generation step (this also stops cleaning of work directory)."),
        ).arg(Arg::with_name("run").short("r").long("run").takes_value(true).help("Automatically runs generated executable."))
        .get_matches();

    let f = fs::read_to_string(file.value_of("file_name").unwrap());

    if f.is_err() {
        println!(
            "Could not read file {}",
            file.value_of("file_name").unwrap()
        )
    } else {
        utils::clean_work();
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
        for tok in parser.clone().tree() {
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
            .map(|item| -> String {
                if item.ext == false {
                    return item.def.clone();
                };
                String::from("")
            })
            .collect();

        let mut extern_defs: Vec<String> = def
            .values()
            .map(|item| -> String {
                if item.ext == true {
                    return item.def.clone();
                };
                String::from("")
            })
            .collect();

        defs.append(&mut main_buffer);

        extern_defs.insert(0, "#include<memory>".to_string());
        extern_defs.insert(0, "#include<vector>".to_string());
        defs.insert(0, "#include \"som_std.cc\"".to_string());

        let joined = defs.join("\n");
        if file.index_of("gen").is_none() {
            utils::make_lib(String::from("som_std"), extern_defs.join("\n"));
            utils::make_work(joined, true);
            if file.index_of("dev-mode").is_none() {
                utils::clean_work();
            }
        } else {
            utils::make_lib(String::from("som_std"), extern_defs.join("\n"));
            utils::make_work(joined, false);
        }
        if file.index_of("run").is_some() {
            let args = file.value_of("run").unwrap();
            utils::run_gen(args.to_string().split(" ").clone().collect());
        }
    }
}
