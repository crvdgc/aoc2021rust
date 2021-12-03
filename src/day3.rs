pub fn part1(input: &str) -> u32 {
    let l: usize = 12;
    let mut n : u32 = 0;
    let mut zero: Vec<u32> = Vec::with_capacity(l);
    for _ in 0..l {
        zero.push(0);
    }
    for line in input.lines() {
        n += 1;
        for (i, c) in line.chars().enumerate() {
            if c == '0' {
                zero[i] += 1;
            }
        }
    }
    let mut base: u32 = 1 << (l - 1);
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    let half = n / 2;
    for i in 0..l {
        if zero[i] > half {
            epsilon += base;
        } else if zero[i] < half {
            gamma += base;
        } else {
            gamma += base;
            epsilon += base;
        }
        base /= 2;
    }
    return gamma * epsilon;
}
