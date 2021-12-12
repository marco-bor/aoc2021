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

    let numbers: Vec<usize> = lines[0]
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let mut boards: Vec<Board> = Vec::new();
    let mut curr_vec: Vec<usize> = Vec::with_capacity(25);

    for i in 2..lines.len() {
        if lines[i].len() == 0 {
            continue;
        }

        curr_vec.extend(
            lines[i]
                .split_whitespace()
                .filter(|s| s.len() > 0)
                .map(|s| s.parse::<usize>().unwrap()),
        );

        if curr_vec.len() == 25 {
            boards.push(Board {
                numbers: curr_vec,
                size: 5,
            });
            curr_vec = Vec::with_capacity(25);
        }
    }

    println!("Task 1 = {}", task1(&numbers, &boards));
    println!("Task 2 = {}", task2(&numbers, &boards));
}

fn task1(numbers: &Vec<usize>, boards: &Vec<Board>) -> usize {
    for i in 0..numbers.len() {
        let extracted_numbers = &numbers[0..=i];
        if let Some(position) = boards.iter().position(|b| b.is_winner(extracted_numbers)) {
            let sum_of_unmarked: usize = boards[position]
                .numbers
                .iter()
                .filter(|x| !extracted_numbers.contains(x))
                .sum();
            return sum_of_unmarked * numbers[i];
        }
    }
    0
}

fn task2(numbers: &Vec<usize>, boards: &Vec<Board>) -> usize {
    let mut remaining_boards: Vec<&Board> = boards.iter().collect();

    for i in 0..numbers.len() {
        let extracted_numbers = &numbers[0..=i];
        if remaining_boards.len() > 1 {
            remaining_boards = remaining_boards
                .into_iter()
                .filter(|b| !b.is_winner(extracted_numbers))
                .collect();
        }

        if remaining_boards.len() == 1 && remaining_boards[0].is_winner(extracted_numbers) {
            let sum_of_unmarked: usize = remaining_boards[0]
                .numbers
                .iter()
                .filter(|x| !extracted_numbers.contains(x))
                .sum();
            return sum_of_unmarked * numbers[i];
        }
    }
    0
}

struct Board {
    numbers: Vec<usize>,
    size: usize,
}

impl Board {
    fn is_winner(&self, checked_numbers: &[usize]) -> bool {
        assert_eq!(self.numbers.len(), self.size * self.size);

        for i in 0..self.size {
            if self.is_row_checked(i, checked_numbers) {
                return true;
            }
            if self.is_column_checked(i, checked_numbers) {
                return true;
            }
        }

        false
    }

    fn is_row_checked(&self, row: usize, checked_numbers: &[usize]) -> bool {
        for i in row * self.size..(row + 1) * self.size {
            if !checked_numbers.contains(&self.numbers[i]) {
                return false;
            }
        }
        true
    }
    fn is_column_checked(&self, column: usize, checked_numbers: &[usize]) -> bool {
        for i in (column..self.numbers.len()).step_by(self.size) {
            if !checked_numbers.contains(&self.numbers[i]) {
                return false;
            }
        }
        true
    }
}
