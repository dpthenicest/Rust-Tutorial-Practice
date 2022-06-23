use std::fs::File;

fn main() {
    // Panic Macro Error Handler
    // panic!("Make i go smack wetin no nice");

    // Sample of programs that'll result in an error
    // let v = vec![0,1,2,3];
    // println!("{}", v[6]);

    // Illustration of the None Option from Option Error Handler
    // let fruits = vec!["banana","apple","coconut"];

    // let first = fruits.get(0);
    // println!("{:?}", first);

    // let third = fruits.get(2);
    // println!("{:?}",third);

    // let non_existent = fruits.get(99);
    // println!("{:?}", non_existent);

    // More complex error handling
    // 1. With match and panic!
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Can't open the file {:?}", error),
    // };

    // 2. With unwrap()
    // let f = File::open("hello.txt").unwrap();

    // 3. With expect()
    let f = File::open("hello.txt").expect("Failed to open hello.txt");


}
