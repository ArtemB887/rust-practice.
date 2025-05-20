use std::io;

fn breaking_records(scores: Vec<i32>) -> Vec<i32> {
    let mut best = scores[0];
    let mut worst = scores[0];
    let mut best_count = 0;
    let mut worst_count = 0;

    for score in scores.iter().skip(1) {
        if *score > best {
            best = *score;
            best_count += 1;
        } else if *score < worst {
            worst = *score;
            worst_count += 1;
        }
    }

    vec![best_count, worst_count]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let scores: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

    let result = breaking_records(scores);
    println!("{} {}", result[0], result[1]);
}
