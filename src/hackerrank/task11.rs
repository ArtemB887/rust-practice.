fn count_apples_and_oranges(s: i32, t: i32, a: i32, b: i32, apples: Vec<i32>, oranges: Vec<i32>) {
    let apple_count = apples.iter().map(|&d| a + d).filter(|&pos| pos >= s && pos <= t).count();
    let orange_count = oranges.iter().map(|&d| b + d).filter(|&pos| pos >= s && pos <= t).count();

    println!("{}", apple_count);
    println!("{}", orange_count);
}

fn main() {
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let st = lines.next().unwrap().unwrap();
    let st: Vec<i32> = st.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (s, t) = (st[0], st[1]);

    let ab = lines.next().unwrap().unwrap();
    let ab: Vec<i32> = ab.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (a, b) = (ab[0], ab[1]);

    let _ = lines.next();

    let apples_line = lines.next().unwrap().unwrap();
    let apples: Vec<i32> = apples_line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let oranges_line = lines.next().unwrap().unwrap();
    let oranges: Vec<i32> = oranges_line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    count_apples_and_oranges(s, t, a, b, apples, oranges);
}
