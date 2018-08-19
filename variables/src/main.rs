fn main() {
    // let x = 5;
    // println!("The value of x is: {}", x);
    // let x = 6;
    // println!("The value of x is: {}", x);
    // let x = x * 2;
    // println!("The value of x is: {}", x);

    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("spaces: {}", spaces);

    // let guess: u32 = "42".parse().expect("Not a number!");

    // let s1 = String::from("Hello");
    // // move s1 into s2. This invalidates s1
    // let s2 = s1;

    // println!("{}, world!", s1);
    // let s1 = String::from("Hello");
    // let s2 = s1.clone();

    // println!("s1 = {0}, s2 = {1}", s1, s2);

    let s = String::from("Hello");
    //After a string moves to a function it becomes invalidated.
    takes_ownership(s);
    // println!("{}", s);
    let x = 5;
    //However, an integer is a copy.
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);

    println!("{}", s3);

    let s1 = String::from("Hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {

    a_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}