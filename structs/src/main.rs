// Structs

// Classic-structs
struct Person {
    name: String,
    age: u8,
    likes_oranges: bool
}

// Tuple-struct
struct Person2D(u32, u32);

fn main() {
    let person = Person {
        name: String::from("Georgina"),
        likes_oranges: true,
        age: 25,
    };

    // Dot Notstion to access it' properties
    println!("Person's name is: {}", person.name);

    // Dot Notation is also used for Tuple-Struct
    let origin = Person2D(25, 55);

    println!("Point contains {:?} and {:?}", origin.0, origin.1);

    // Destructuring Tuple-Struct from 'origin'
    let Person2D(x, y) = origin;
    println!("The Points are: {:?} and {:?}", x, y);
}
