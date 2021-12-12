use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|x| x.expect("Error when reading line"))
        .collect();

    println!("Task 1 = {}", task1());
    println!("Task 2 = {}", task2());
}

fn task1() -> usize {
    0
}

fn task2() -> usize {
    0
}
