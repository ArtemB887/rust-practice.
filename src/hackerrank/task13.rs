use std::io;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

fn gcd_list(arr: &[i32]) -> i32 {
    arr.iter().copied().reduce(|a, b| gcd(a, b)).unwrap()
}

fn lcm_list(arr: &[i32]) -> i32 {
    arr.iter().copied().reduce(|a, b| lcm(a, b)).unwrap()
}

fn between_two_sets(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let lcm_a = lcm_list(&a);
    let gcd_b = gcd_list(&b);
    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).unwrap();
    let _ = input1;

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).unwrap();
    let a: Vec<i32> = input2.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).unwrap();
    let b: Vec<i32> = input3.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();

    let result = between_two_sets(a, b);
    println!("{}", result);
}
