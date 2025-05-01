use std::io;

fn time_conversion(s: &str) -> String {
    let hour: u32 = s[0..2].parse().unwrap();
    let is_pm = &s[8..] == "PM";
    let mut new_hour = hour;

    if is_pm && hour != 12 {
        new_hour += 12;
    } else if !is_pm && hour == 12 {
        new_hour = 0;
    }

    format!("{:02}{}", new_hour, &s[2..8])
}

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let result = time_conversion(s.trim());
    println!("{}", result);
}
