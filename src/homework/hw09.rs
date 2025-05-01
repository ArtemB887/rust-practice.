pub fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }

    let shift = ((n % len as isize + len as isize) % len as isize) as usize;

    let chars: Vec<char> = s.chars().collect();
    let rotated: String = chars[len - shift..]
        .iter()
        .chain(&chars[..len - shift])
        .collect();
    
    rotated
}

#[cfg(test)]
mod tests {
    use super::rotate;

    #[test]
    fn test() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        for (n, exp) in shifts.iter() {
            assert_eq!(rotate(s.clone(), *n), exp.to_string());
        }
    }
}
