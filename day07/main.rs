use std::convert::TryInto;
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
    let positions: Vec<isize> = lines[0]
        .split(',')
        .map(|x: &str| x.parse().unwrap())
        .collect();

    println!("Task 1 = {}", task1(positions.as_slice()));
    println!("Task 2 = {}", task2(positions.as_slice()));
}

fn task1(positions: &[isize]) -> usize {
    let mut min_i = -1;
    let mut min = 0;
    for i in 0..=*positions.iter().max().unwrap() {
        let fuel_total = fuel(positions, i);
        if fuel_total < min || min_i == -1 {
            min_i = i;
            min = fuel_total;
        }
    }
    min
}

fn task2(positions: &[isize]) -> usize {
    let mut min_i = -1;
    let mut min = 0;

    assert_eq!(sum_of_first_n_numbers(16 - 5), 66);
    for i in 0..=*positions.iter().max().unwrap() {
        let fuel_total = fuel2(positions, i);
        if fuel_total < min || min_i == -1 {
            min_i = i;
            min = fuel_total;
        }
    }
    min
}

fn fuel(positions: &[isize], move_to: isize) -> usize {
    positions
        .into_iter()
        .map(|x| (x - move_to).abs())
        .sum::<isize>() as usize
}

fn fuel2(positions: &[isize], move_to: isize) -> usize {
    positions
        .into_iter()
        .map(|x| sum_of_first_n_numbers((x - move_to).abs().try_into().unwrap()))
        .sum::<usize>() as usize
}

fn sum_of_first_n_numbers(n: usize) -> usize {
    return (n * (n + 1)) / 2;
}
