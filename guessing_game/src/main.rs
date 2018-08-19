//To obtain user input, we're bringing in the standard I/O library.
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

//entry point into the program
fn main() {
    //Macro that prints a string to the screen.
    println!("Guess the number!");

    let secret_number = rand::thread_rng()
                        .gen_range(1, 101);



    

    // println!("The secret number was: {}", secret_number);

loop{
    println!("Please input your guess.");

    //Creating a place to store the user input.
    // let creates a variable.
    // mut is mutable.
    // let mut guess is creating a variable that is mutable 
    // with a new instance of a string.
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess : u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("You guessed: {}", guess);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
