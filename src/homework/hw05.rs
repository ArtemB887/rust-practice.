// Function to compute the greatest common divisor using Euclidean algorithm
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Tests to verify correctness
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_cases() {
        let cases = [
            ((24, 60), 12),
            ((15, 9), 3),
            ((15, 6), 3),
            ((140, 40), 20),
            ((24, 16), 8),
            ((100, 10), 10),
            ((120, 80), 40),
            ((80, 120), 40),
            ((100, 20), 20),
            ((37, 11), 1),
            ((120, 90), 30),
        ];

        for ((a, b), expected) in cases {
            assert_eq!(gcd(a, b), expected);
        }
    }
}
