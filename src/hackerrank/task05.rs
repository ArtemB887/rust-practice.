use std::io::{self, BufRead};

fn plus_minus(arr: &[i32]) {
    let total = arr.len() as f64;
    let positives = arr.iter().filter(|&&x| x > 0).count() as f64;
    let negatives = arr.iter().filter(|&&x| x < 0).count() as f64;
    let zeros = arr.iter().filter(|&&x| x == 0).count() as f64;

    println!("{:.6}", positives / total);
    println!("{:.6}", negatives / total);
    println!("{:.6}", zeros / total);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    lines.next();
    let arr: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    plus_minus(&arr);
}
