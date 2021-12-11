use itertools::Itertools;

pub fn part1(note: &str) -> u32 {
    let mut acc = 0;
    for entry in note.lines() {
        acc += count_simple_digits(&entry);
    }
    return acc;
}

fn count_simple_digits(entry: &str) -> u32 {
    let mut acc = 0;
    for digit in entry.split('|').nth(1).unwrap().trim().split(' ') {
        let l = digit.len();
        if l == 7 || l == 4 || l == 3 || l == 2 {
            acc += 1;
        }
    }
    acc
}

pub fn part2(note: &str) -> u32 {
    let mut sum = 0;
    for entry in note.lines() {
        let mut iter = entry.split('|');
        let input = iter.next().unwrap().trim();
        let output = iter.next().unwrap().trim();
        println!("input: {}, output: {}", input, output);
        let map = solve_input(&input);
        sum += decode_output(map, &output);
    }
    sum
}

fn decode_output(map: [char; 7], output: &str) -> u32 {
    let mut base = 1000;
    let mut sum = 0;
    println!("output: {}", output);
    for digit in output.split(' ') {
        println!("digit: {}", digit);
        let mut decoded = String::new();
        for c in digit.chars() {
            match c {
                'a' => decoded.push(map[0]),
                'b' => decoded.push(map[1]),
                'c' => decoded.push(map[2]),
                'd' => decoded.push(map[3]),
                'e' => decoded.push(map[4]),
                'f' => decoded.push(map[5]),
                'g' => decoded.push(map[6]),
                _ => panic!("illegal"),
            }
        }
        println!("decoded: {}", decoded);
        let n = match decoded.chars().sorted().collect::<String>().as_str() {
            "abcefg" => 0,
            "cf" => 1,
            "acdeg" => 2,
            "acdfg" => 3,
            "bcdf" => 4,
            "abdfg" => 5,
            "abdefg" => 6,
            "acf" => 7,
            "abcdefg" => 8,
            "abcdfg" => 9,
            _ => panic!("not digit"),
        };
        sum += n * base;
        base /= 10;
    }

    sum
}

fn is_1(x: String) -> bool {
    x.len() == 2
}

fn is_4(x: String) -> bool {
    x.len() == 4
}

fn is_7(x: String) -> bool {
    x.len() == 3
}

fn split_out<T: Clone>(xs: &mut Vec<T>, p: &dyn Fn(T) -> bool) -> Vec<T> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < xs.len() {
        if p(xs[i].clone()) {
            out.push(xs.remove(i));
        } else {
            i += 1;
        }
    }
    out
}

fn filter_out(from: &String, pattern: &String) -> String {
    from.chars().filter(|x| !pattern.contains(*x)).collect()
}

fn len_5(x: String) -> bool {
    x.len() == 5
}

/*           -   -     - -
 * digit len a b c d e f g
 *   2    5  x   x x x   x
 *   3    5  x   x     x x
 *   5    5  x x   x   x x
 *   0    6  x x x   x x x
 *   6    6  x x   x x x x
 *   9    6  x x x x   x x
 * */

fn solve_input(input: &str) -> [char; 7] {
    let mut digits: Vec<String> = input
        .split(' ')
        .filter(|x| x.len() != 7) // remove digit 8
        .map(|digit| digit.chars().sorted().collect::<String>())
        .collect_vec();
    println!("{:?}", digits);
    // digit 1 -> c f
    let cf = split_out(&mut digits, &is_1)[0].clone();
    println!("cf -> {}", cf);
    // digit 7 -> a c f
    let acf = split_out(&mut digits, &is_7)[0].clone();
    println!("acf -> {}", acf);
    let a = filter_out(&acf, &cf).chars().next().unwrap();
    println!("a -> {}", a);
    let bcdf = split_out(&mut digits, &is_4)[0].clone();
    println!("bcdf -> {}", bcdf);
    let bd = filter_out(&bcdf, &cf);
    println!("bd -> {}", bd);
    let mut bd_iter = bd.chars();
    let bd0 = bd_iter.next().unwrap();
    let bd1 = bd_iter.next().unwrap();
    let mut d235 = split_out(&mut digits, &len_5);
    let mut i = 0;
    // digit 5 -> b d f g
    let mut abdfg: String = "xxx".to_owned();
    while i < d235.len() {
        if d235[i].contains(bd0) && d235[i].contains(bd1) {
            abdfg = d235.remove(i);
        } else {
            i += 1;
        }
    }
    println!("abdfg -> {}", abdfg);
    let fg = filter_out(&abdfg, &bd).chars().filter(|x| *x != a).collect::<String>();
    println!("fg -> {}", fg);
    let f = cf.chars().filter(|x| fg.contains(*x)).next().unwrap();
    println!("f -> {}", f);
    let c = cf.chars().filter(|x| *x != f).next().unwrap();
    println!("c -> {}", c);
    let g = fg.chars().filter(|x| *x != f).next().unwrap();
    println!("g -> {}", g);
    // digit 2 -> a c d e g
    i = 0;
    let mut acdeg: String = "xxx".to_owned();
    while i < d235.len() {
        if !d235[i].contains(f) {
            acdeg = d235.remove(i);
        } else {
            i += 1;
        }
    }
    println!("acdeg -> {}", acdeg);
    let mut acg = String::new();
    acg.push(a);
    acg.push(c);
    acg.push(g);
    let de = filter_out(&acdeg, &acg);
    println!("de -> {}", de);
    let d = bd.chars().filter(|x| de.contains(*x)).next().unwrap();
    println!("d -> {}", d);
    let b = bd.chars().filter(|x| *x != d).next().unwrap();
    println!("b -> {}", b);
    let e = de.chars().filter(|x| *x != d).next().unwrap();
    println!("e -> {}", e);
    let rmap = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let mut map = ['x'; 7];
    for (i, ch) in (vec![a, b, c, d, e, f, g]).iter().enumerate() {
        map[find_index(*ch)] = rmap[i];
    }
    println!("{:?}", map);
    map
}

fn find_index(ch: char) -> usize {
    match ch {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        _ => panic!("not index"),
    }
}
