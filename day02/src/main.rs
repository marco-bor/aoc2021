use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    let state = &mut State::default();

    for command in input.lines().flat_map(|s| s.parse::<Command>()) {
        state.exec(command);
    }

    (state.x * state.y) as usize
}

fn problem2(input: &str) -> usize {
    let state = &mut State1::default();

    for command in input.lines().flat_map(|s| s.parse::<Command>()) {
        state.exec(command);
    }

    (state.x * state.y) as usize
}

#[test]
fn test_problem1() {
    assert_eq!(problem1(include_str!("test_input.txt")), 150)
}

#[test]
fn test_problem2() {
    assert_eq!(problem2(include_str!("test_input.txt")), 900)
}

enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((command_str, amount)) = s.split_once(' ') else {
            return Err(());
        };
        let Ok(amount) = amount.parse::<usize>() else {
            return Err(());
        };
        match command_str {
            "forward" => Ok(Command::Forward(amount)),
            "down" => Ok(Command::Down(amount)),
            "up" => Ok(Command::Up(amount)),
            _ => Err(()),
        }
    }
}

trait ExecCommand {
    fn exec(&mut self, command: Command);
}

#[derive(Default)]
struct State {
    x: i32,
    y: i32,
}

impl ExecCommand for State {
    fn exec(&mut self, command: Command) {
        match command {
            Command::Forward(x) => self.x += x as i32,
            Command::Down(x) => self.y += x as i32,
            Command::Up(x) => self.y -= x as i32,
        }
    }
}

#[derive(Default)]
struct State1 {
    x: i32,
    y: i32,
    aim: i32,
}

impl ExecCommand for State1 {
    fn exec(&mut self, command: Command) {
        match command {
            Command::Forward(x) => {
                self.x += x as i32;
                self.y += self.aim * x as i32;
            }
            Command::Down(x) => self.aim += x as i32,
            Command::Up(x) => self.aim -= x as i32,
        }
    }
}
