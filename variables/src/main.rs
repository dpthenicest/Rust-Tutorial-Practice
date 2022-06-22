fn main() {
    let mut x = 1;
    println!("The value of x: {}", x);
    x = 2;
    println!("The value of x: {}", x);
    let y: bool = true;
    println!("The value of y: {}", y);
    let y: bool = false;
    println!("The value of y: {}", y);

    const STRING: &str = "hello";
    println!("The value of the string constant is: {}", STRING)
}