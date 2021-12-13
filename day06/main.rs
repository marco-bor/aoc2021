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

    let lanternfish: Vec<usize> = lines[0]
        .split(',')
        .map(|x: &str| x.parse().unwrap())
        .collect();

    println!("Task 1 = {}", task1(&lanternfish));
    println!("Task 2 = {}", task2(lanternfish.as_slice()));
}

fn task1(input: &Vec<usize>) -> usize {
    let mut lanternfish: Vec<usize> = input.clone();
    for _ in 0..80 {
        for i in 0..lanternfish.len() {
            if lanternfish[i] == 0 {
                lanternfish.push(8);
                lanternfish[i] = 6;
            } else {
                lanternfish[i] -= 1;
            }
        }
    }
    lanternfish.len()
}

fn task2(input: &[usize]) -> u128 {
    // Vector index -> lanternfish timer value
    // Vector value -> lanterfish count
    let mut lanternfish = [0u128; 9];

    for timer in input {
        lanternfish[timer.clone()] += 1;
    }

    for day in 0..256 {
        let zero = lanternfish[0];
        lanternfish[0] = lanternfish[1];
        lanternfish[1] = lanternfish[2];
        lanternfish[2] = lanternfish[3];
        lanternfish[3] = lanternfish[4];
        lanternfish[4] = lanternfish[5];
        lanternfish[5] = lanternfish[6];
        lanternfish[6] = zero + lanternfish[7];
        lanternfish[7] = lanternfish[8];
        lanternfish[8] = zero;
    }

    count(lanternfish)
}

fn count(lanternfish: [u128; 9]) -> u128 {
    let mut count = 0u128;

    for fish_count in lanternfish {
        count += fish_count;
    }

    count
}
