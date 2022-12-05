fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    let cols = input.lines().next().unwrap().len();
    let rows = input.lines().count();

    let mut gamma_str = String::with_capacity(cols);
    let mut epsilon_str = String::with_capacity(cols);

    for i in 0..cols {
        let occurrence_of_1 = count_occurrences_in_column(input.lines(), i, '1');
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

fn problem2(input: &str) -> usize {
    co2_scrubber_rating(input) * oxigen_generator_rating(input)
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("test_input.txt")), 198)
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("test_input.txt")), 230)
}

fn oxigen_generator_rating(input: &str) -> usize {
    let cols = input.lines().next().unwrap().len();

    let mut filtered_report: Vec<&str> = input.lines().collect();

    for i in 0..cols {
        let occurrence_of_1 = count_occurrences_in_column(filtered_report.iter(), i, '1');
        let occurrence_of_0 = filtered_report.len() - occurrence_of_1;

        if occurrence_of_1 >= occurrence_of_0 {
            filtered_report.retain(|s| s.chars().nth(i).unwrap() == '1');
        } else {
            filtered_report.retain(|s| s.chars().nth(i).unwrap() == '0');
        }

        if filtered_report.len() == 1 {
            return usize::from_str_radix(filtered_report[0], 2).unwrap();
        }
    }
    0
}

fn co2_scrubber_rating(input: &str) -> usize {
    let cols = input.lines().next().unwrap().len();

    let mut filtered_report: Vec<&str> = input.lines().collect();

    for i in 0..cols {
        let occurrence_of_1 = count_occurrences_in_column(filtered_report.iter(), i, '1');
        let occurrence_of_0 = filtered_report.len() - occurrence_of_1;

        if occurrence_of_0 <= occurrence_of_1 {
            filtered_report.retain(|s| s.as_bytes()[i] as char == '0');
        } else {
            filtered_report.retain(|s| s.as_bytes()[i] as char == '1');
        }

        if filtered_report.len() == 1 {
            return usize::from_str_radix(filtered_report[0], 2).unwrap();
        }
    }
    0
}

fn count_occurrences_in_column(
    input: impl Iterator<Item = impl AsRef<str>>,
    column: usize,
    character: char,
) -> usize {
    input
        .map(|line| line.as_ref().as_bytes()[column] as char)
        .filter(|&c| c == character)
        .count()
}
