fn main() {
    // Constants for the envelope size
    const WIDTH: usize = 12;
    const HEIGHT: usize = 7;

    // Draw the top part of the envelope
    print!("*");
    for _ in 1..WIDTH - 1 {
        print!(" ");
    }
    println!("*");

    // Draw the middle part of the envelope
    for row in 1..HEIGHT - 1 {
        print!("*");
        for _ in 1..WIDTH - 2 * row {
            print!(" ");
        }
        println!("*");
    }

    // Draw the bottom part of the envelope
    print!("*");
    for _ in 1..WIDTH - 1 {
        print!(" ");
    }
    println!("*");
}
