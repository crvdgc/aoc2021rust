use std::fs;

fn main() {
    let input = fs::read_to_string("data/day2.txt").expect("File not exist");
    let res1 = day2_part1(&input);
    println!("part1: {}", res1);
    let res2 = day2_part2(&input);
    println!("part2: {}", res2);
}

enum Command {
    Forward,
    Up,
    Down,
}

fn day2_part1(input: &str) -> i32 {
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    for command in input.lines() {
        match parse_command(command) {
            Some((Command::Forward, x)) => {
                horizontal += x;
            }
            Some((Command::Up, x)) => {
                depth -= x;
            }
            Some((Command::Down, x)) => {
                depth += x;
            }
            None => {
                panic!("Unrecognized command");
            }
        }
    }
    return depth * horizontal;
}

fn day2_part2(input: &str) -> i32 {
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    let mut aim: i32 = 0;
    for command in input.lines() {
        match parse_command(command) {
            Some((Command::Forward, x)) => {
                horizontal += x;
                depth += x * aim;
            }
            Some((Command::Up, x)) => {
                aim -= x;
            }
            Some((Command::Down, x)) => {
                aim += x;
            }
            None => {
                panic!("Unrecognized command");
            }
        }
    }
    return depth * horizontal;
}

fn parse_command(command: &str) -> Option<(Command, i32)> {
    if let Some(value) = command.strip_prefix("forward ") {
        return Some((Command::Forward, parse_value(value)));
    } else if let Some(value) = command.strip_prefix("up ") {
        return Some((Command::Up, parse_value(value)));
    } else if let Some(value) = command.strip_prefix("down ") {
        return Some((Command::Down, parse_value(value)));
    } else {
        return None;
    }
}

fn parse_value(value: &str) -> i32 {
   return value.parse::<i32>().unwrap();
}
