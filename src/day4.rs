use std::str::Lines;

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let iter = &mut input.lines();
    let draw_numbers = parse_draw_numbers(iter.next().unwrap());
    let mut boards: Vec<Board> = Vec::new();
    while let Some(_) = iter.next() {
        boards.push(Board::new(parse_board(iter)));
    }
    // println!("{:?}", boards);
    for number in draw_numbers.iter() {
        // println!("{}", number);
        for board in boards.iter_mut() {
            board.mark(*number);
            // println!("{:?}", board.is_win());
            if board.is_win() {
                return number * board.unmarked_sum();
            }
        }
    }
    panic!("no winner");
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let iter = &mut input.lines();
    let draw_numbers = parse_draw_numbers(iter.next().unwrap());
    let mut boards: Vec<Board> = Vec::new();
    while let Some(_) = iter.next() {
        boards.push(Board::new(parse_board(iter)));
    }
    let mut last_score = 0;
    for number in draw_numbers.iter() {
        for board in boards.iter_mut() {
            if !board.has_won {
                board.mark(*number);
                if board.is_win() {
                    last_score = number * board.unmarked_sum();
                    board.has_won = true;
                }
            }
        }
    }
    return last_score;
}

#[derive(Debug)]
struct Board {
    has_won: bool,
    numbers: Vec<u32>,
    marked: Vec<bool>,
}

impl Board {
    fn new(numbers: Vec<u32>) -> Self {
        let mut marked = Vec::with_capacity(25);
        for _ in 0..25 {
            marked.push(false);
        }
        Self { numbers, marked, has_won: false }
    }

    fn is_win(&self) -> bool {
        for i in 0..5 {
            let mut row_win = true;
            let mut col_win = true;
            for j in 0..5{
                row_win = row_win && self.marked[i * 5 + j];
                col_win = col_win && self.marked[j * 5 + i];
            }
            if row_win || col_win {
                return true;
            }
        }
        return false;
    }

    fn mark(&mut self, number: u32) {
        for i in 0..5 {
            for j in 0..5 {
                let idx = i * 5 + j;
                if self.numbers[idx] == number {
                    self.marked[idx] = true;
                }
            }
        }
    }

    fn unmarked_sum(&self) -> u32 {
        let mut sum: u32 = 0;
        for (is_marked, v) in self.marked.iter().zip(self.numbers.iter()) {
            if !*is_marked {
                sum += v;
            }
        }
        return sum;
    }
}

// parsing
fn parse_draw_numbers(s: &str) -> Vec<u32>{
    let mut v: Vec<u32> = Vec::new();
    for n in s.split(',') {
        v.push(n.parse().unwrap());
    }
    return v;
}

fn parse_board(iter: &mut Lines) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    for row in iter.take(5) {
        for n in row.split_whitespace() {
            v.push(n.parse().unwrap());
        }
    }
    return v;
}
