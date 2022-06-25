fn main() {
    let text = "Hello\nWorld\n!";
    let upper = text.to_uppercase();
    let stripped = upper.strip_prefix("HELLO\n").unwrap();
    println!("{}", first_line(stripped));
}

pub fn first_line(string: &str) -> &str {
    string.lines().next().unwrap()
}

/* There are two types of string: 
    1. String(An owned string): This owns the string data. Data freed when dropped. has three parts: length, capacity and data pointer. 
    2. &str(A borrowed string slice): This does not own the string data. Data not freed when dropped. Has two parts: length and data pointer.*/  