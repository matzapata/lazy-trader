use terminal_size::{terminal_size, Width};

pub fn print_divider() {
    let width = match terminal_size() {
        Some((Width(w), _)) => w as usize,
        None => 80, // Default to 80 if size can't be determined
    };
    println!("{}", "-".repeat(width));
}
