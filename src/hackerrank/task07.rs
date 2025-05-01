use std::io::{self, BufRead};

fn mini_max_sum(arr: &[u64]) {
    let total_sum: u64 = arr.iter().sum();
    let min = arr.iter().min().unwrap();
    let max = arr.iter().max().unwrap();
    println!("{} {}", total_sum - max, total_sum - min);
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line).unwrap();
    let arr: Vec<u64> = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    mini_max_sum(&arr);
}
