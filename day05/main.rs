use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader, Result};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);

    let lines: Vec<Line> = reader
        .lines()
        .map(|x: Result<String>| x.expect("Error when reading line"))
        .map(|s: String| Line::from(s.as_str()))
        .collect();

    println!("Task 1 = {}", task1(lines.as_slice()));
    println!("Task 2 = {}", task2(lines.as_slice()));
}

fn task1(lines: &[Line]) -> usize {
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

    let mut count = 0;

    //dump_matrix(&matrix, width, height);

    for x in 0..width {
        for y in 0..height {
            if matrix[x][y] >= 2 {
                count += 1;
            }
        }
    }

    count
}

fn task2(lines: &[Line]) -> usize {
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

    let mut count = 0;

    //dump_matrix(&matrix, width, height);

    for x in 0..width {
        for y in 0..height {
            if matrix[x][y] >= 2 {
                count += 1;
            }
        }
    }

    count
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

impl From<&str> for Line {
    fn from(s: &str) -> Line {
        let (a_str, b_str) = s.split_once(" -> ").expect("Invalid format");
        let a = Point::from(a_str);
        let b = Point::from(b_str);
        if a.x > b.x {
            Line { a: b, b: a }
        } else {
            Line { a: a, b: b }
        }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Point {
        let (x_str, y_str) = s.split_once(',').expect("Expected ',' char");
        Point {
            x: x_str.parse().expect("Invalid x coordinate"),
            y: y_str.parse().expect("Invalid y coordinate"),
        }
    }
}

fn dump_matrix(matrix: &Vec<Vec<usize>>, width: usize, height: usize) {
    for x in 0..width {
        for y in 0..height {
            if matrix[y][x] == 0 {
                print!(". ");
            } else {
                print!("{} ", matrix[y][x]);
            }
        }
        print!("\n");
    }
    std::io::stdout().flush();
}
