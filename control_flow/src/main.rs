fn main() {

    if 1 == 2 {
        println!("Decisions are taken when one is vexed!");
    } else {
        println!("Everything's cool now, proceed to take the decisions.");
    };

    // The results of if else is stored in the greeting variable and can be used later
    let formal: bool = true;
    let greeting: () = if formal {
        println!("Good evening");
    } else {
        println!("Hey, friend");
    };

    // Combine multiple if else
    let number: i32 = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by either 4 or 3");
    };

    // match is the same as switch case
    let boolean: bool = true;

    let binary =  match boolean {
        false => 0,
        true => 1,
    };
    println!("{:?} is equal to {:?}", boolean, binary)
}
