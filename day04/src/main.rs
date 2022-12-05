use std::str::FromStr;

const SIZE: usize = 5;

fn main() {
    let input = include_str!("input.txt");

    println!("Task 1 = {}", problem1(input));
    println!("Task 2 = {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    let Input { numbers, boards } = input.parse::<Input>().unwrap();
    for i in 0..numbers.len() {
        let extracted_numbers = &numbers[0..=i];
        if let Some(position) = boards.iter().position(|b| b.is_winner(extracted_numbers)) {
            let sum_of_unmarked: usize = boards[position]
                .numbers
                .iter()
                .filter(|x| !extracted_numbers.contains(x))
                .map(|&n| n as usize)
                .sum();
            return sum_of_unmarked * numbers[i] as usize;
        }
    }
    0
}

fn problem2(input: &str) -> usize {
    let Input { numbers, boards } = input.parse::<Input>().unwrap();
    let mut remaining_boards: Vec<&Board> = boards.iter().collect();

    for i in 0..numbers.len() {
        let extracted_numbers = &numbers[0..=i];
        if remaining_boards.len() > 1 {
            remaining_boards.retain(|b| !b.is_winner(extracted_numbers));
        }

        if remaining_boards.len() == 1 && remaining_boards[0].is_winner(extracted_numbers) {
            let sum_of_unmarked: usize = remaining_boards[0]
                .numbers
                .iter()
                .filter(|x| !extracted_numbers.contains(x))
                .map(|&n| n as usize)
                .sum();
            return sum_of_unmarked * numbers[i] as usize;
        }
    }
    0
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("test_input.txt")), 4512)
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("test_input.txt")), 1924)
}

struct Input {
    numbers: Vec<u8>,
    boards: Vec<Board>,
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line_iter = s.split("\n\n");
        let numbers: Vec<u8> = line_iter
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        let boards: Vec<Board> = line_iter.flat_map(|s| s.parse::<Board>()).collect();
        Ok(Input { numbers, boards })
    }
}

struct Board {
    numbers: Vec<u8>,
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Board {
            numbers: s.split([' ', '\n']).flat_map(|s| s.parse::<u8>()).collect(),
        })
    }
}

impl Board {
    fn is_winner(&self, checked_numbers: &[u8]) -> bool {
        for i in 0..SIZE {
            if self.is_row_checked(i, checked_numbers) {
                return true;
            }
            if self.is_column_checked(i, checked_numbers) {
                return true;
            }
        }

        false
    }

    fn is_row_checked(&self, row: usize, checked_numbers: &[u8]) -> bool {
        for i in row * SIZE..(row + 1) * SIZE {
            if !checked_numbers.contains(&self.numbers[i]) {
                return false;
            }
        }
        true
    }
    fn is_column_checked(&self, column: usize, checked_numbers: &[u8]) -> bool {
        for i in (column..self.numbers.len()).step_by(SIZE) {
            if !checked_numbers.contains(&self.numbers[i]) {
                return false;
            }
        }
        true
    }
}
