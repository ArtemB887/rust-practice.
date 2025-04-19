// Constants for the width and height of the diamond
const WIDTH: usize = 11;
const HEIGHT: usize = 11;

fn main() {
    let mut output = String::new();

    // Upper part of the diamond
    for i in 0..HEIGHT / 2 + 1 {
        for _ in 0..(HEIGHT / 2 - i) {
            output.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            output.push('*');
        }
        output.push('\n');
    }

    // Lower part of the diamond
    for i in (0..HEIGHT / 2).rev() {
        for _ in 0..(HEIGHT / 2 - i) {
            output.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            output.push('*');
        }
        output.push('\n');
    }

    // Single println! as required
    println!("{}", output);
}
