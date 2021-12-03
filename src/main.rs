use std::fs;
mod day3;

fn main() {
    // day2
    // let input = fs::read_to_string("data/day2.txt").expect("File not exist");
    // let res1 = day2::part1(&input);
    // println!("part1: {}", res1);
    // let res2 = day2_part2(&input);
    // println!("part2: {}", res2);

    // day3
    let input = fs::read_to_string("data/day3.txt").expect("File not exist");
    println!("{}", day3::part1(&input, 12));
    println!("{}", day3::part2(&input, 12));
}
