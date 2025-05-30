use std::io;

fn divisible_sum_pairs(k: i32, ar: Vec<i32>) -> i32 {
    let mut count = 0;
    let n = ar.len();
    for i in 0..n {
        for j in i+1..n {
            if (ar[i] + ar[j]) % k == 0 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let _n = parts[0];
    let k = parts[1];

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let ar: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let result = divisible_sum_pairs(k, ar);
    println!("{}", result);
}
