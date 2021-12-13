use itertools::Itertools;

pub fn part1(input: &str) -> u32 {
    let mut energy = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars().map(|x| x.to_digit(10).unwrap()) {
            row.push(c);
        }
        energy.push(row);
    }
    // println!("{:?}", map);
    let mut sum = 0;
    for _ in 0..100 {
        sum += step(&mut energy);
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    let mut energy = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars().map(|x| x.to_digit(10).unwrap()) {
            row.push(c);
        }
        energy.push(row);
    }
    // println!("{:?}", map);
    let mut day = 1;
    loop {
        if step2(&mut energy) {
            break;
        } else {
            day += 1;
        }
    }
    day
}

fn init_shone(energy: &mut Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    energy.iter().map(|row| { row.iter().map(|_| { false }).collect_vec() }).collect_vec()
}


fn step(energy: &mut Vec<Vec<u32>>) -> u32 {
    let mut flashes = 0;
    let mut shone = init_shone(energy);
    let h = energy.len();
    let w = energy[0].len();
    for i in 0..h {
        for j in 0..w {
            energy[i][j] += 1
        }
    }
    let mut has_flash = true;
    while has_flash {
        has_flash = false;
        for i in 0..h {
            for j in 0..w {
                if !shone[i][j] && energy[i][j] > 9 {
                    has_flash = true;
                    flashes += 1;
                    shone[i][j] = true;
                    energy[i][j] = 0;
                    for k in [-1, 0, 1] {
                        for l in [-1, 0, 1] {
                            let ii = i as i32 + k;
                            let jj = j as i32 + l;
                            if ii >= 0 && ii < h as i32 && jj >= 0 && jj < w as i32 {
                                if !shone[ii as usize][jj as usize] {
                                    energy[ii as usize][jj as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    flashes
}

fn step2(energy: &mut Vec<Vec<u32>>) -> bool {
    let mut flashes = 0;
    let mut shone = init_shone(energy);
    let h = energy.len();
    let w = energy[0].len();
    for i in 0..h {
        for j in 0..w {
            energy[i][j] += 1
        }
    }
    let mut has_flash = true;
    while has_flash {
        has_flash = false;
        for i in 0..h {
            for j in 0..w {
                if !shone[i][j] && energy[i][j] > 9 {
                    has_flash = true;
                    flashes += 1;
                    shone[i][j] = true;
                    energy[i][j] = 0;
                    for k in [-1, 0, 1] {
                        for l in [-1, 0, 1] {
                            let ii = i as i32 + k;
                            let jj = j as i32 + l;
                            if ii >= 0 && ii < h as i32 && jj >= 0 && jj < w as i32 {
                                if !shone[ii as usize][jj as usize] {
                                    energy[ii as usize][jj as usize] += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    shone.iter().all(|row| row.iter().all(|x| *x))
}

