use itertools::Itertools;

pub fn part1(input: &str) -> u32 {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars().map(|x| x.to_digit(10).unwrap()) {
            row.push(c);
        }
        map.push(row);
    }
    let mut sum = 0;
    let h = map.len();
    let w = map[0].len();
    for i in 0..h {
        for j in 0..w {
            if is_lowest(&map, i, j) {
                sum += 1 + map[i][j];
            }
        }
    }
    // println!("{:?}", map);
    sum
}

pub fn part2(input: &str) -> u32 {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars().map(|x| x.to_digit(10).unwrap()) {
            row.push(c);
        }
        map.push(row);
    }
    let mut basins = Vec::new();
    let mut visited = to_visited(&map);
    let h = map.len();
    let w = map[0].len();
    for i in 0..h {
        for j in 0..w {
            if is_lowest(&map, i, j) {
                basins.push(find_basin(&mut visited, i, j));
            }
        }
    }
    // println!("{:?}", map);
    basins.sort();
    basins.reverse();
    basins[0] * basins[1] * basins[2]
}

fn is_lowest(map: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let cur = map[x][y];
    let h = map.len();
    let w = map[0].len();
    let mut lowest = true;
    for i in [-1, 1] {
        let xx = x as i32 + i;
        if xx >= 0 && xx < h as i32 {
            lowest = lowest && cur < map[xx as usize][y];
        }
        let yy = y as i32 + i;
        if yy >= 0 && yy < w as i32 {
            lowest = lowest && cur < map[x][yy as usize];
        }
    }
    if lowest {
        println!("({}, {}): {}", x, y, map[x][y]+ 1)
    }
    lowest
}

fn to_visited(map: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    map.iter().map(|row| {
        row.iter().map(|x| {
            if *x == 9 {
                true
            } else {
                false
            }
        }).collect_vec()
    }).collect_vec()
}

fn find_basin(visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> u32 {
    let h = visited.len();
    let w = visited[0].len();
    visited[x][y] = true;
    let mut acc = 1;
    for i in [-1, 1] {
        let xx = x as i32 + i;
        if xx >= 0 && xx < h as i32 && !visited[xx as usize][y] {
            acc += find_basin(visited, xx as usize, y);
        }
        let yy = y as i32 + i;
        if yy >= 0 && yy < w as i32 && !visited[x][yy as usize] {
            acc += find_basin(visited, x, yy as usize);
        }
    }
    acc
}
