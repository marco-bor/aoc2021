use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    let lines: Vec<Line> = input.lines().flat_map(|s| s.parse::<Line>()).collect();

    let width = 1 + lines
        .iter()
        .map(|line: &Line| std::cmp::max(line.a.x, line.b.x))
        .max()
        .unwrap();
    let height = 1 + lines
        .iter()
        .map(|line: &Line| std::cmp::max(line.a.y, line.b.y))
        .max()
        .unwrap();

    let matrix = &mut vec![vec![0usize; height]; width];

    for line in lines {
        if line.is_vertical() {
            let start_y = std::cmp::min(line.a.y, line.b.y);
            let end_y = std::cmp::max(line.a.y, line.b.y);

            for y in start_y..=end_y {
                matrix[line.a.x][y] += 1;
            }
        } else if line.is_horizontal() {
            let start_x = std::cmp::min(line.a.x, line.b.x);
            let end_x = std::cmp::max(line.a.x, line.b.x);

            for x in start_x..=end_x {
                matrix[x][line.a.y] += 1;
            }
        }
    }

    matrix
        .iter()
        .flat_map(|m| m.iter())
        .filter(|&&n| n >= 2)
        .count()
}

fn problem2(input: &str) -> usize {
    let lines: Vec<Line> = input.lines().flat_map(|s| s.parse::<Line>()).collect();

    let width = 1 + lines
        .iter()
        .map(|line: &Line| std::cmp::max(line.a.x, line.b.x))
        .max()
        .unwrap();
    let height = 1 + lines
        .iter()
        .map(|line: &Line| std::cmp::max(line.a.y, line.b.y))
        .max()
        .unwrap();

    let matrix = &mut vec![vec![0usize; height]; width];

    for line in lines {
        if line.is_vertical() {
            let start_y = std::cmp::min(line.a.y, line.b.y);
            let end_y = std::cmp::max(line.a.y, line.b.y);

            for y in start_y..=end_y {
                matrix[line.a.x][y] += 1;
            }
        } else if line.is_horizontal() {
            let start_x = std::cmp::min(line.a.x, line.b.x);
            let end_x = std::cmp::max(line.a.x, line.b.x);

            for x in start_x..=end_x {
                matrix[x][line.a.y] += 1;
            }
        } else {
            let step_y: isize =
                std::cmp::min(1, std::cmp::max(-1, line.b.y as isize - line.a.y as isize));
            let start_x = std::cmp::min(line.a.x, line.b.x);
            let end_x = std::cmp::max(line.a.x, line.b.x);

            for x in start_x..=end_x {
                matrix[x][(line.a.y as isize + (x - start_x) as isize * step_y) as usize] += 1;
            }
        }
    }

    matrix
        .iter()
        .flat_map(|m| m.iter())
        .filter(|&&n| n >= 2)
        .count()
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("test_input.txt")), 5)
}
#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("test_input.txt")), 12)
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }
    fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" -> ").expect("Invalid format");
        let (a, b) = (a.parse::<Point>().unwrap(), b.parse::<Point>().unwrap());
        if a.x > b.x {
            Ok(Line { a: b, b: a })
        } else {
            Ok(Line { a, b })
        }
    }
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(',').expect("Expected ',' char");
        Ok(Point {
            x: x_str.parse().expect("Invalid x coordinate"),
            y: y_str.parse().expect("Invalid y coordinate"),
        })
    }
}
