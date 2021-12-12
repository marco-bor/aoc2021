use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[derive(Debug, Default)]
struct State {
    x: i32,
    y: i32,
    aim: i32,
}

impl State {
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

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);

    let commands: Vec<Command> = reader
        .lines()
        .map(|s| string_to_command(&s.unwrap()).unwrap())
        .collect();

    let state = &mut State::default();

    for command in commands {
        state.exec(command);
    }

    println!("{:?}", state);
}

fn string_to_command(input: &str) -> Option<Command> {
    let split_result: Vec<&str> = input.split_whitespace().take(2).collect();
    let command_str = split_result[0];
    let parsed_amount = split_result[1].parse::<usize>().unwrap();
    match command_str {
        "forward" => Some(Command::Forward(parsed_amount)),
        "down" => Some(Command::Down(parsed_amount)),
        "up" => Some(Command::Up(parsed_amount)),
        _ => None,
    }
}
