use std::collections::BTreeSet;

pub fn part1(input: &str) -> u32 {
    parse_input(input);
    0
}

fn parse_point(line: &str) -> Option<(u32, u32)> {
    let mut numbers = line.split(',').into_iter();
    let vx = numbers.next()?;
    let x = vx.parse().ok()?;
    let vy = numbers.next()?;
    let y = vy.parse().ok()?;
    Some((x, y))
}

fn parse_fold(line: &str) -> Option<(bool, u32)> {
    let mut i = line.split('=');
    let dir = i.next()?;
    let d = dir.chars().rev().next()?;
    let position = i.next()?;
    let n = position.parse().ok()?;
    Some((d == 'x', n ))
}

fn parse_input(input: &str) -> (BTreeSet<(u32, u32)>, Vec<(bool, u32)>) {
    let mut points = BTreeSet::new();
    let mut folds = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if let Some(point) = parse_point(line) {
            println!("{:?}", point);
            points.insert(point);
        } else {
            if let Some(fold) = parse_fold(line) {
                println!("{:?}", fold);
                folds.push(fold);
            }
        }
    }
    (points, folds)
}

fn fold_along((isX: bool, axis: u32), BTreeSet<(u32, )>
