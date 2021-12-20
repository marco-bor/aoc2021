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

    let outputs: Vec<Vec<&str>> = lines
        .iter()
        .map(|s: &String| {
            let (_, out_str) = s.split_once(" | ").unwrap();
            out_str.split_whitespace().collect()
        })
        .collect();

    println!("Task 1 = {}", task1(&outputs));
    println!("Task 2 = {}", task2(&outputs));
}

fn task1(outputs: &Vec<Vec<&str>>) -> usize {
    outputs
        .iter()
        .map(|out: &Vec<&str>| {
            out.iter()
                .filter(|s| s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7)
                .count()
        }) // convert &Vec<&str> to usize
        .sum::<usize>()
}

fn task2(outputs: &Vec<Vec<&str>>) -> usize {
    for out in outputs {
        println!(
            "{:?}: {}",
            out,
            vec_to_decimal(
                &out.iter()
                    .map(|s| pattern_to_number(s).unwrap())
                    .collect::<Vec<_>>()
            )
        );
    }
    outputs
        .iter()
        .map(|out: &Vec<&str>| {
            vec_to_decimal(
                &out.iter()
                    .map(|s| pattern_to_number(s).unwrap())
                    .collect::<Vec<_>>(),
            )
        }) // convert &Vec<&str> to usize
        .sum::<usize>()
}

// 1,3,2,1 -> 1231
fn vec_to_decimal(out: &Vec<usize>) -> usize {
    out.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, num)| acc + 10usize.pow(i as u32) * num)
}

fn pattern_to_number(pattern: &str) -> Option<usize> {
    if pattern.len() == 2 {
        return Some(1);
    }
    if pattern.len() == 4 {
        return Some(4);
    }
    if pattern.len() == 3 {
        return Some(7);
    }
    if pattern.len() == 7 {
        return Some(8);
    }

    if pattern.len() == 5 {
        // can be 5, 2 or 3
        if !pattern.contains('a') {
            return Some(5);
        }
        if pattern.contains('g') {
            return Some(2);
        }
        return Some(3);
    }
    if pattern.len() == 6 {
        // can be 0, 6 or 9
        if !pattern.contains('a') {
            return Some(6);
        }
        if !pattern.contains('g') {
            return Some(9);
        }
        return Some(0);
    }
    return None;
}
