use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);

    let matrix: Vec<Vec<u32>> = reader
        .lines()
        .map(|x| x.expect("Error when reading line"))
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).expect("Error when parsing line"))
                .collect()
        })
        .collect();

    println!("Task 1 = {}", task1(&matrix));
    println!("Task 2 = {}", task2());
}

fn task1(matrix: &Vec<Vec<u32>>) -> usize {
    let width = matrix[0].len();
    println!("matrix len = {}", matrix.len());
    let mut low_points: Vec<u32> = Vec::new();
    for y in 0..matrix.len() {
        let is_first_row = y == 0;
        let is_last_row = y == matrix.len() - 1;

        for x in 0..width {
            let is_last_col = x == width - 1;
            let is_first_col = x == 0;

            let value = matrix[y][x];

            // Exit early if value is greater than (or equal) adjacents

            if !is_first_col && value >= matrix[y][x - 1] {
                continue;
            }

            if !is_last_col && value >= matrix[y][x + 1] {
                continue;
            }

            if !is_first_row && value >= matrix[y - 1][x] {
                continue;
            }
            if !is_last_row && value >= matrix[y + 1][x] {
                continue;
            }

            low_points.push(value);
        }
    }

    low_points.iter().sum::<u32>() as usize + low_points.len()
}

fn task2() -> usize {
    0
}
