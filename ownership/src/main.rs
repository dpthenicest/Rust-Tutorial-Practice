fn main() {
    // Demonstration of the concept of ownerahip, moves the pointer of the value stored from a variable to another variable
    let say = String::from("Cat");
    let say2 = say;
    println!("{}", say2);

    // Concept of cloning, but this creates a new data/value in the heap instead of a pointer
    let say3 = String::from("Dog");
    let say4 = say3.clone();
    println!("{}", say3);

    // When 'say3' is dropped it doesn't affect 'say4'
    drop(say3);
    println!("{}", say4);
}
