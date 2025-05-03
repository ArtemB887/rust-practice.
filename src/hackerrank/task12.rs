use std::io;

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> &'static str {
    if v1 == v2 {
        if x1 == x2 {
            "YES"
        } else {
            "NO"
        }
    } else {
        let numerator = x1 - x2;
        let denominator = v2 - v1;
        if denominator != 0 && numerator % denominator == 0 && (numerator / denominator) >= 0 {
            "YES"
        } else {
            "NO"
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = kangaroo(parts[0], parts[1], parts[2], parts[3]);
    println!("{}", result);
}
