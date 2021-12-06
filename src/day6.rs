pub fn part1(input: &str) -> u128 {
    let mut fish_bin = parse_fish(input.trim());
    for _ in 0..256 {
        new_day(&mut fish_bin);
    }
    fish_bin.iter().sum()
}

fn parse_fish(input: &str) -> [u128; 9] {
    let mut fish_bin = [0; 9];
    for scan in input.split(',') {
        let timer: usize = scan.parse().unwrap();
        fish_bin[timer] += 1;
    }
    fish_bin
}

fn new_day(fish_bin: &mut [u128; 9]) {
    let new_fish = fish_bin[0];
    for i in 0..8 {
        fish_bin[i] = fish_bin[i+1];
    }
    fish_bin[6] += new_fish;
    fish_bin[8] = new_fish;
}
