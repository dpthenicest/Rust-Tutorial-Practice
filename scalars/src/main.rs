fn main() {
    // Integers
    let a: u8 = 5;
    let b: u8 = 1;

    // Floating Points
    let x = 2.0;
    let y: f32 = 3.0;

    // Numerical Operations
    let sum = a + b;
    println!("The sum of a and b is: {}", sum);

    let difference = x - 1.0;
    println!("The difference of x and 1.0 is: {}", difference);

    let product = 4 * a;
    println!("The product of 4 and a is: {}", product);

    let quotient = 9.0 / y;
    println!("9.0 quotient y is: {}", quotient);

    let remainder = a % b;
    println!("a remainder b is: {}", remainder);

    // Boolean: true or false
    let t: bool = true;
    let f: bool = false;
    println!("The two booleans are: {} and {}", t, f);

    // Character
    let c = '😒';
    let d = 'è';
    println!("The characters were defined namely; {} and {}", c, d);
}
