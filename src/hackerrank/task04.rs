use std::io;

fn diagonal_difference(matrix: &Vec<Vec<i32>>) -> i32 {
    let n = matrix.len();
    let mut primary = 0;
    let mut secondary = 0;

    for i in 0..n {
        primary += matrix[i][i];
        secondary += matrix[i][n - 1 - i];
    }

    (primary - secondary).abs()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut matrix = Vec::with_capacity(n);
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        matrix.push(row);
    }

    let result = diagonal_difference(&matrix);
    println!("{}", result);
}
