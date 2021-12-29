use nanoid::nanoid;
use std::env;
use std::fs;
use std::process::Command;

pub fn make_work(content: String) {
    let mut dir = env::temp_dir();
    dir.push("something_work");
    fs::create_dir(&dir);

    dir.push("module.cc");
    fs::write(&dir, content);
    // g++ -Wall -o main main.cpp -static
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("som.out");
    Command::new("g++")
        .args([
            "-Wall",
            "-o",
            current_dir.to_str().unwrap(),
            dir.to_str().unwrap(),
            "-static",
        ])
        .spawn()
        .expect("Failed to build");
    dir.pop();
    fs::remove_dir(dir);
}
