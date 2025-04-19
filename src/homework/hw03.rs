fn main() {
    // Constants for the envelope size
    const WIDTH: usize = 12;  // Width
    const HEIGHT: usize = 7;  // Height

    // Draw the top part of the envelope
    println!("****************************");

    for i in 1..HEIGHT {
        let spaces = WIDTH - i * 2;  // Number of spaces between the stars
        let inner_spaces = (WIDTH - 2 * i) / 2;  // Spaces for centering

        // Left part of the envelope
        print!("* ");
        for _ in 0..i {
            print!(" ");
        }
        // Centered block of stars
        for _ in 0..spaces {
            print!(" ");
        }
        // Right part of the envelope
        print!("*");
        println!();
    }

    // Draw the bottom part of the envelope
    for _ in 0..WIDTH * 2 {
        print!("*");
    }
    println!();
}
