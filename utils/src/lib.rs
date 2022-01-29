use nanoid::nanoid;
use std::process::{Command, Stdio};

use std::env;
use std::fs;

pub fn make_work(content: String, compile: bool) {
    let mut dir = env::temp_dir();
    dir.push("something_work");
    if dir.is_dir() == false {
        fs::create_dir(&dir).expect("Failed to create work directory");
    }

    dir.push("module.cc");
    fs::write(&dir, content).expect("Failed to write module");
    // g++ -Wall -o main main.cpp -static
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("som.out");
    if compile {
        let mut build = Command::new("g++")
            .args([
                "-o",
                current_dir.to_str().unwrap(),
                dir.to_str().unwrap(),
                "-static",
            ])
            .spawn()
            .expect("Failed to build");
        build.wait().expect("Failed to build");
    }
}

pub fn clean_work() {
    let mut dir = env::temp_dir();
    dir.push("something_work");
    if dir.is_dir() == true {
        fs::remove_dir_all(dir).expect("Failed to clean up work directory");
    }
}

pub fn run_gen(args: Vec<&str>) {
    let child = Command::new("./som.out")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let output = child.unwrap().wait_with_output().unwrap();

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout);
        println!("{}", raw_output.unwrap());
    } else {
        let out = String::from_utf8(output.stdout);
        println!("{}", out.unwrap());

        let err = String::from_utf8(output.stderr);
        println!("{}", err.unwrap());
        std::process::exit(1);
    }
}

pub fn make_lib(name: String, content: String) {
    let mut dir = env::temp_dir();
    dir.push("something_work");
    if dir.is_dir() == false {
        fs::create_dir(&dir).expect("Failed to create work directory");
    }
    dir.push(name.clone() + ".cc");
    fs::write(&dir, content).expect("Failed to make library.");
}
