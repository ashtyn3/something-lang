use nanoid::nanoid;
use std::env;
use std::fs;
use std::process::Command;

pub fn make_work(content: String, compile: bool) {
    let mut dir = env::temp_dir();
    dir.push("something_work");
    fs::create_dir(&dir);

    dir.push("module.cc");
    fs::write(&dir, content);
    // g++ -Wall -o main main.cpp -static
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("som.out");
    if compile {
        Command::new("g++")
            .args([
                "-o",
                current_dir.to_str().unwrap(),
                dir.to_str().unwrap(),
                "-static",
            ])
            .spawn()
            .expect("Failed to build");
    }
}

pub fn clean_work() {
    let mut dir = env::temp_dir();
    dir.push("something_work");
    fs::remove_dir(dir).expect("Failed to clean up work directory");
}
