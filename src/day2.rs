enum Command {
    Forward,
    Up,
    Down,
}

pub fn part1(input: &str) -> i32 {
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

pub fn part2(input: &str) -> i32 {
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

