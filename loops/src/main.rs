fn main() {
    // Loops iterate forever until it breaks with a break keyword
    let mut i: i32 = 1;
    let something: i32 = loop {
        i *= 2;
        if i > 100 {
            break i;
        }
    };
    // assert_eq!(something, 128);
    println!("{}",something);

    // while Iteration: stops when a condition is met
    let mut counter: i32 = 0;

    while counter < 10 {
        println!("Hello {}", counter);
        counter += 1;
    };

    // for loop: preferred iteration for selections of items in an array, etc.
    for item in 0..5 {
        println!("{}", item*2);
    };
}
