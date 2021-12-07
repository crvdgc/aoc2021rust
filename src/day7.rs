pub fn part1(input: &str) -> u32 {
    let mut crabs = parse_crabs(&input);
    crabs.sort();
    let median = crabs[crabs.len() / 2];
    return crabs
        .iter()
        .map(|crab|(*crab as i32 - median as i32).abs() as u32)
        .sum();
}

pub fn part2(input: &str) -> u32 {
    let mut crabs = parse_crabs(&input);
    let mut least_fuel = 1 << 30;
    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();
    for position in *min..=*max {
        let fuel = crabs
            .iter()
            .map(|crab| part2_fuel(*crab, position))
            .sum();
        println!("{}: {}", position, fuel);
        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }
    least_fuel

}

fn parse_crabs(input: &str) -> Vec<u32> {
    let mut crabs: Vec<u32> = Vec::new();
    for crab in input.trim().split(',') {
        crabs.push(crab.parse().unwrap());
    }
    crabs
}

fn part2_fuel(crab: u32, target: u32) -> u32 {
    let dist = (crab as i32 - target as i32).abs() as u32;
    (1 + dist) * dist / 2
}
