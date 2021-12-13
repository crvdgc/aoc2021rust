pub fn part1(input: &str) -> u32 {
    let mut score = 0;
    for line in input.lines() {
        // println!("{}", line);
        let mut stack = Vec::new();
        score += transition(&mut stack, &mut line.chars().into_iter());
    }
    score
}

fn transition<I>(stack: &mut Vec<char>, iter: &mut I) -> u32
    where I: Iterator<Item = char> {
    if let Some(c) = iter.next() {
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
                transition(stack, iter)
            },
            ')' => check(stack, '(', iter),
            ']' => check(stack, '[', iter),
            '}' => check(stack, '{', iter),
            '>' => check(stack, '<', iter),
            _ => panic!("impossible"),
        }
    } else { // input exhausted
        0
    }
}

fn check<I>(stack: &mut Vec<char>, expected: char, iter: &mut I)  -> u32
    where I: Iterator<Item = char> {
    if let Some(c) = stack.pop() {
        if c == expected {
            transition(stack, iter)
        } else {
            match expected {
                '(' => 3,
                '[' => 57,
                '{' => 1197,
                '<' => 25137,
                _ => panic!("bad stack"),
            }
        }
    } else { // incomplete
        0
    }
}

pub fn part2(input: &str) -> u128 {
    let mut scores = Vec::new();
    for line in input.lines() {
        // println!("{}", line);
        let mut stack = Vec::new();
        let score =  transition2(&mut stack, &mut line.chars().into_iter());
        if score != 0 {
            scores.push(score);
        }
    }
    scores.sort();
    println!("{:?}", scores);
    scores[scores.len() / 2]
}

fn transition2<I>(stack: &mut Vec<char>, iter: &mut I) -> u128
    where I: Iterator<Item = char> {
    if let Some(c) = iter.next() {
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
                transition2(stack, iter)
            },
            ')' => check2(stack, '(', iter),
            ']' => check2(stack, '[', iter),
            '}' => check2(stack, '{', iter),
            '>' => check2(stack, '<', iter),
            _ => panic!("impossible"),
        }
    } else { // input exhausted
        if !stack.is_empty() {
            clean_stack(stack)
        } else { // correct
            0
        }
    }
}

fn check2<I>(stack: &mut Vec<char>, expected: char, iter: &mut I)  -> u128
    where I: Iterator<Item = char> {
    if let Some(c) = stack.pop() {
        if c == expected {
            transition2(stack, iter)
        } else { // corrupted, discard
            0
        }
    } else { // incomplete
        panic!("bad input");
    }
}

fn clean_stack(stack: &mut Vec<char>) -> u128 {
    println!("{:?}", stack);
    let mut score = 0;
    while !stack.is_empty() {
        if let Some(c) = stack.pop() {
            score *= 5;
            score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("impossible")
            }
        } 
    }
    score
}
