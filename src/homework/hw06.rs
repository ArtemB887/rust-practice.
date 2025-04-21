fn draw_tree(triangles: usize) {
    let mut output = String::new();
    let max_width = 2 * (triangles - 1) + 1;

    // Draw the triangles
    for level in 1..=triangles {
        for i in 0..level {
            let stars = 2 * i + 1;
            let spaces = (max_width - stars) / 2;

            let line: String = std::iter::repeat(' ')
                .take(spaces).collect::<String>()
                + &std::iter::repeat('*')
                    .take(stars).collect::<String>();

            output.push_str(&line);
            output.push('\n');
        }
    }

    // Draw the trunk
    let trunk_spaces = (max_width - 1) / 2;
    let trunk_line = std::iter::repeat(' ')
        .take(trunk_spaces).collect::<String>() + "*\n";
    for _ in 0..triangles {
        output.push_str(&trunk_line);
    }

    // One single print!
    print!("{output}");
}

fn main() {
    let triangles = 5; // number of triangle layers
    draw_tree(triangles);
}
