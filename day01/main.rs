use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);

    let numbers: Vec<usize> = reader
        .lines()
        .map(|line| line.unwrap().parse::<usize>().unwrap())
        .collect();

    let a = String::from("aaaaa");

    println!("{}", count_increases(&numbers));
    println!("{}", count_increases_by_summing_3(&numbers));
}

fn count_increases(values: &[usize]) -> usize {
    values.windows(2).filter(|&x| x[0] < x[1]).count()
}

fn count_increases_by_summing_3(values: &[usize]) -> usize {
    let sum_of_3: Vec<usize> = values.windows(3).map(|x| x.iter().sum()).collect();
    count_increases(&sum_of_3)
}
