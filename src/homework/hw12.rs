pub fn count_permutation(shipments: &Vec<u32>) -> isize {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();
    if total as usize % n != 0 {
        return -1;
    }
    let average = total / n as u32;
    let mut moves = 0;
    for &weight in shipments {
        if weight > average {
            moves += (weight - average) as usize;
        }
    }
    moves as isize
}

use rand::Rng;

pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let average = rng.gen_range(1..=10);
    let mut result = vec![average; n];
    for i in 0..n / 2 {
        let delta = rng.gen_range(1..=average.min(5));
        result[i] += delta;
        result[n - 1 - i] = result[n - 1 - i].saturating_sub(delta);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let shipments = vec![1, 1, 1, 1, 6];
        assert_eq!(count_permutation(&shipments), 4);
    }

    #[test]
    fn test_impossible_case() {
        let shipments = vec![1, 2, 3];
        assert_eq!(count_permutation(&shipments), -1);
    }

    #[test]
    fn test_gen_shipments_validity() {
        let n = 6;
        let shipments = gen_shipments(n);
        let total: u32 = shipments.iter().sum();
        assert_eq!(total as usize % n, 0);
    }
}
