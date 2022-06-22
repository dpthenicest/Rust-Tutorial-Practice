// Functions
fn main() {
    let x = last_char(String::from(""));
    println!("Value of string: {}", x);
}

fn last_char(string: String) -> char {
    if string.is_empty() {
        return '🎑';
    }
    string.chars().next_back().unwrap()
}
