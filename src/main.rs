use std::fs;
mod day8;

fn main() {
    // day2
    // let input = fs::read_to_string("data/day2.txt").expect("File not exist");
    // let res1 = day2::part1(&input);
    // println!("part1: {}", res1);
    // let res2 = day2_part2(&input);
    // println!("part2: {}", res2);

    // day3
    // let input = fs::read_to_string("data/day3.txt").expect("File not exist");
    // println!("{}", day3::part1(&input, 12));
    // println!("{}", day3::part2(&input, 12));

    // day4
    // let input = fs::read_to_string("data/day4.txt").expect("File not exist");
    // println!("{}", day4::part1(&input));
    // println!("{}", day4::part2(&input));

    // day5
    // let input = fs::read_to_string("data/day5.txt").expect("File not exist");
    // println!("{}", day5::part1(&input));

    // day6
    // let input = fs::read_to_string("data/day6.txt").expect("File not exist");
    // println!("{}", day6::part1(&input));

    // day 7
    // let input = fs::read_to_string("data/day7.txt").expect("File not exist");
    // println!("{}", day7::part2(&input));

    // day8
    let input = fs::read_to_string("data/day8.txt").expect("File not exist");
    // println!("{}", day8::part1(&input));
    println!("{}", day8::part2(&input));
}
