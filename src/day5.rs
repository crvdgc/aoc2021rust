use std::convert::TryInto;
use std::collections::BTreeMap;

pub fn part1(input: &str) -> u32 {
    let mut points: BTreeMap<(u32, u32), u32> = BTreeMap::new();
    for line in input.lines() {
        let cur_line = parse_line(line);
        for point in cur_line.points() {
            if points.contains_key(&point) {
                points.insert(point, points[&point] + 1);
            } else {
                points.insert(point, 1);
            }
        }
    }
    points.retain(|_, lines| *lines >= 2);
    return points.len() as u32;
}

#[derive(Debug)]
struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

impl Line {

    fn new(sx: u32, sy: u32, ex: u32, ey: u32) -> Self {
        Line {
            start: (sx, sy),
            end: (ex, ey),
        }
    }

    fn points(&self) -> Vec<(u32, u32)> {
        let dxy: (i32, i32);
        let mut points = Vec::new();
        let sx = self.start.0 as i32;
        let sy = self.start.1 as i32;
        let ex = self.end.0 as i32;
        let ey = self.end.1 as i32;
        if sx == ex { // vertical
            dxy = (0, signum(ey - sy));
        }  else if sy == ey { // horizontal
            dxy = (signum(ex - sx), 0);
        } else if ex - sx == ey - sy { // diagonal
            let slope = signum(ex - sx);
            dxy = (slope, slope);
        } else if ex + ey == sx + sy { // anti-diagonal
            let slope = signum(ex - sx);
            dxy = (slope, -slope);
        } else {
            return points;
        }
        let mut p = self.start;
        while p != self.end {
            // println!("{:?}", p);
            points.push(p);
            let new_x = (p.0 as i32 + dxy.0).try_into().unwrap();
            let new_y = (p.1 as i32 + dxy.1).try_into().unwrap();
            p = (new_x, new_y);
        }
        points.push(self.end);
        return points;

    }
}

fn signum(x: i32)  -> i32 {
    if x > 0 {
        return 1;
    } else if x < 0 {
        return -1;
    } else {
        return 0;
    }
}

fn parse_line(s: &str) -> Line {
    let mut points = s.split(" -> ");
    let mut start = points.next().unwrap().split(',');
    let mut end = points.next().unwrap().split(',');
    let s0 = start.next().unwrap().parse().unwrap();
    let s1 = start.next().unwrap().parse().unwrap();
    let e0 = end.next().unwrap().parse().unwrap();
    let e1 = end.next().unwrap().parse().unwrap();
    Line::new(s0, s1, e0, e1)
}
