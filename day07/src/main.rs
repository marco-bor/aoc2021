fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

type Number = i32;
type NumberAbs = u32;

fn problem1(input: &str) -> Number {
    let positions: Vec<Number> = input.split(',').flat_map(|x| x.parse::<Number>()).collect();

    let mut min = fuel(&positions, 0);
    for i in 1..=*positions.iter().max().unwrap() {
        let fuel_total = fuel(&positions, i);
        if fuel_total < min {
            min = fuel_total;
        }
    }
    min
}

fn problem2(input: &str) -> Number {
    let positions: Vec<Number> = input.split(',').flat_map(|x| x.parse::<Number>()).collect();

    let mut min = fuel2(&positions, 0);

    for i in 1..=*positions.iter().max().unwrap() {
        let fuel_total = fuel2(&positions, i);
        if fuel_total < min {
            min = fuel_total;
        }
    }
    min
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("test_input.txt")), 37);
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("test_input.txt")), 168);
}

fn fuel(positions: &[Number], move_to: Number) -> Number {
    positions.iter().map(|&x| x.abs_diff(move_to)).sum::<u32>() as Number
}

fn fuel2(positions: &[Number], move_to: Number) -> Number {
    positions
        .iter()
        .map(|&x| sum_of_first_n_numbers(x.abs_diff(move_to)))
        .sum::<NumberAbs>() as Number
}

const fn sum_of_first_n_numbers(n: NumberAbs) -> NumberAbs {
    (n * (n + 1)) / 2
}

#[test]
fn test_sum_of_first_n_numbers() {
    assert_eq!(sum_of_first_n_numbers(16 - 5), 66);
}
