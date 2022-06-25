fn main() {
    // Demonstration of Borrowing: creates a reference to the value
    let say = String::from("Cat");
    print_out(&say);
    println!("Again: {}", say);

    // Mutable Borrowing
    let mut my_vec = vec![1 ,2, 3];
    println!("{:?}", my_vec);
    // You'll need to also add a reference to the 'mut' keyword when passing the variable as an argument
    add_to_vec(&mut my_vec);
    println!("{:?}", my_vec);

    /* Valid References: A value is no longer valid after it is dropped. Any reference to that value also become invalid when it is dropped*/
    let say1 = String::from("Dog");
    let say2 = &say1;
    println!("{}", say1);
    drop(say1);
    // This will throw an error because it has exceeded the lifetime of 'say1'
    // println!("{}", say2);
}

fn print_out(to_print: &String) {
    println!("{}", to_print);
}

// You'll need to add a reference to the 'mut' keyword in the parameter
fn add_to_vec(a_vec: &mut Vec<i32>) {
    a_vec.push(4);
}
