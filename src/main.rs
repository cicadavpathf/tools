use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line.\n");
    let input_str = input.trim_end();

    let dot_split: Vec<&str> = input_str.split('.').filter(|&s| !s.is_empty()).collect();
    let integer_part = dot_split.first().unwrap_or(&"0");
    let fractional_part = dot_split.last().unwrap_or(&"0");

    println!("{}.{}", integer_part, fractional_part);
}
