fn main() {
    let input = include_str!("input.txt");

    println!("Task 1 = {}", problem1(input));
    println!("Task 2 = {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| l.split_once(" | "))
        .map(|out| {
            out.1
                .split_whitespace()
                .filter(|s| s.len() == 2 || s.len() == 4 || s.len() == 3 || s.len() == 7)
                .count()
        })
        .sum::<usize>()
}

fn problem2(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| l.split_once(" | "))
        .map(|(signals, output)| {
            let one = find_by_len(signals, 2);
            let four = find_by_len(signals, 4);
            output
                .split_whitespace()
                .map(|s| pattern_to_number(s, one, four).unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| vec_to_decimal(&v))
        .sum::<usize>()
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("test_input.txt")), 26)
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("test_input.txt")), 61229)
}

// 1,3,2,1 -> 1321
fn vec_to_decimal(out: &[usize]) -> usize {
    out.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, num)| acc + 10usize.pow(i as u32) * num)
}
#[test]
fn test_vec_to_decimal() {
    assert_eq!(vec_to_decimal(&[]), 0);
    assert_eq!(vec_to_decimal(&[2]), 2);
    assert_eq!(vec_to_decimal(&[1, 3, 2, 1]), 1321);
}

fn find_by_len(signals: &str, len: usize) -> &str {
    signals
        .split_whitespace()
        .filter(|s| s.len() == len)
        .next()
        .unwrap()
}

fn count_overlaps(a: &str, b: &str) -> usize {
    a.chars().filter(|&c| b.contains(c)).count()
}

fn pattern_to_number(pattern: &str, one: &str, four: &str) -> Option<usize> {
    match pattern.len() {
        2 => return Some(1),
        3 => return Some(7),
        4 => return Some(4),
        7 => return Some(8),
        5 => {
            if count_overlaps(pattern, one) == 2 {
                return Some(3);
            } else if count_overlaps(pattern, four) == 2 {
                return Some(2);
            } else {
                return Some(5);
            }
        }
        6 => {
            if count_overlaps(pattern, four) == 4 {
                return Some(9);
            } else if count_overlaps(pattern, one) == 2 {
                return Some(0);
            } else {
                return Some(6);
            }
        }
        _ => return None,
    }
}
