use std::io::{self, BufRead};

fn birthday_cake_candles(candles: &[u32]) -> usize {
    let max_height = candles.iter().max().unwrap();
    candles.iter().filter(|&&c| c == *max_height).count()
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    lines.next();
    let candles: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let result = birthday_cake_candles(&candles);
    println!("{}", result);
}
