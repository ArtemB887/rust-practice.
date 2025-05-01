use std::io;

fn simple_array_sum(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let result = simple_array_sum(&arr);
    println!("{}", result);
}
