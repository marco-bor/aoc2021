fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    let mut lanternfish: Vec<u8> = input.split(',').flat_map(|x| x.parse()).collect();
    for _ in 0..80 {
        for i in 0..lanternfish.len() {
            if lanternfish[i] == 0 {
                lanternfish.push(8);
                lanternfish[i] = 6;
            } else {
                lanternfish[i] -= 1;
            }
        }
    }
    lanternfish.len()
}

fn problem2(input: &str) -> u128 {
    // Vector index -> lanternfish timer value
    // Vector value -> lanterfish count
    let mut lanternfish = [0u128; 9];

    for timer in input.split(',').flat_map(|x| x.parse::<u8>()) {
        lanternfish[timer as usize] += 1;
    }

    for _day in 0..256 {
        let zero = lanternfish[0];
        lanternfish[0] = lanternfish[1];
        lanternfish[1] = lanternfish[2];
        lanternfish[2] = lanternfish[3];
        lanternfish[3] = lanternfish[4];
        lanternfish[4] = lanternfish[5];
        lanternfish[5] = lanternfish[6];
        lanternfish[6] = zero + lanternfish[7];
        lanternfish[7] = lanternfish[8];
        lanternfish[8] = zero;
    }

    lanternfish.iter().sum()
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("test_input.txt")), 5934);
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("test_input.txt")), 26984457539);
}
