fn birthday(s: Vec<i32>, d: i32, m: i32) -> i32 {
    let mut count = 0;
    for i in 0..=s.len() - m as usize {
        if s[i..i + m as usize].iter().sum::<i32>() == d {
            count += 1;
        }
    }
    count
}

fn main() {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    lines.next();
    let s: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace().map(|x| x.parse().unwrap()).collect();
    let dm: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace().map(|x| x.parse().unwrap()).collect();

    let result = birthday(s, dm[0], dm[1]);
    println!("{}", result);
}
