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

    println!("Task 1 = {}", task1(&lines));
    println!("Task 2 = {}", task2(&lines));
}

fn task1(report: &Vec<String>) -> usize {
    let cols = report[0].len();
    let rows = report.len();

    let mut gamma_str = String::with_capacity(cols);
    let mut epsilon_str = String::with_capacity(cols);

    for i in 0..cols {
        let occurrence_of_1 = count_occurrences_in_column(&report, i, '1');
        let occurrence_of_0 = rows - occurrence_of_1;

        if occurrence_of_1 > occurrence_of_0 {
            gamma_str.push('1');
            epsilon_str.push('0');
        } else {
            gamma_str.push('0');
            epsilon_str.push('1');
        }
    }

    let gamma = usize::from_str_radix(&gamma_str, 2).expect("Gamma cannot be converted to decimal");
    let epsilon =
        usize::from_str_radix(&epsilon_str, 2).expect("Epsilon cannot be converted to decimal");

    gamma * epsilon
}

fn task2(report: &Vec<String>) -> usize {
    co2_scrubber_rating(report) * oxigen_generator_rating(report)
}

fn oxigen_generator_rating(report: &Vec<String>) -> usize {
    let cols = report[0].len();

    let mut filtered_report: Vec<&String> = report.iter().collect();

    for i in 0..cols {
        let occurrence_of_1 = count_occurrences_in_column1(&filtered_report, i, '1');
        let occurrence_of_0 = filtered_report.len() - occurrence_of_1;

        if occurrence_of_1 >= occurrence_of_0 {
            filtered_report = filtered_report
                .into_iter()
                .filter(|s| s.chars().nth(i).unwrap() == '1')
                .collect();
        } else {
            filtered_report = filtered_report
                .into_iter()
                .filter(|s| s.chars().nth(i).unwrap() == '0')
                .collect();
        }

        if filtered_report.len() == 1 {
            return usize::from_str_radix(filtered_report[0], 2).unwrap();
        }
    }
    0
}

fn co2_scrubber_rating(report: &Vec<String>) -> usize {
    let cols = report[0].len();

    let mut filtered_report: Vec<&String> = report.iter().collect();

    for i in 0..cols {
        let occurrence_of_1 = count_occurrences_in_column1(&filtered_report, i, '1');
        let occurrence_of_0 = filtered_report.len() - occurrence_of_1;

        if occurrence_of_0 <= occurrence_of_1 {
            filtered_report = filtered_report
                .into_iter()
                .filter(|s| s.chars().nth(i).unwrap() == '0')
                .collect();
        } else {
            filtered_report = filtered_report
                .into_iter()
                .filter(|s| s.chars().nth(i).unwrap() == '1')
                .collect();
        }

        if filtered_report.len() == 1 {
            return usize::from_str_radix(filtered_report[0], 2).unwrap();
        }
    }
    0
}

fn get_nth_char(line: &str, n: usize) -> char {
    line.chars()
        .nth(n)
        .expect(&format!("line has no index {}", n))
}

fn count_occurrences_in_column(rows: &Vec<String>, column: usize, character: char) -> usize {
    rows.iter().fold(0, |p, line| {
        p + ((get_nth_char(&line, column) == character) as usize)
    })
}

fn count_occurrences_in_column1(rows: &Vec<&String>, column: usize, character: char) -> usize {
    rows.iter().fold(0, |p, line| {
        p + ((get_nth_char(&line, column) == character) as usize)
    })
}
