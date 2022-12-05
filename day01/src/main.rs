fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    parse_input(input)
        .windows(2)
        .filter(|&x| x[0] < x[1])
        .count()
}

fn problem2(input: &str) -> usize {
    parse_input(input)
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|&x| x[0] < x[1])
        .count()
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .flat_map(|line| line.parse::<usize>())
        .collect::<Vec<usize>>()
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("test_input.txt")), 7)
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("test_input.txt")), 5)
}
