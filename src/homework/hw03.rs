// hw03.rs

const WIDTH: usize = 30;
const HEIGHT: usize = 14;

fn main() {
    let mut output = String::new();

    for y in 0..=HEIGHT {
        for x in 0..=WIDTH {
            let ch = if y == 0 || y == HEIGHT {
                // Top and bottom borders
                '*'
            } else if x == 0 || x == WIDTH {
                // Left and right borders
                '*'
            } else if x == 2 * y || x == WIDTH - 2 * y {
                // Envelope diagonals
                '*'
            } else {
                // Empty space inside
                ' '
            };
            output.push(ch);
        }
        output.push('\n');
    }

    print!("{}", output);
}
