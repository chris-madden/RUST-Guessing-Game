// Bring io library into scope
// io library is comes from standard library
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// Entry point into the program
// fn declares new function
fn main() {

    // this is a macro
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("Please input your guess.");

        // 'let' is used to create a variable
        // Variables are immutable by default
        // 'mut' allows the variable to be mutable
        // 'String' is a variable type
        // '::' indicated new is an associated function of the String type
        // Some languages call this a static method
        let mut guess = String::new();

        // Call the stdin() function from the io library
        io::stdin()
            .read_line(&mut guess) // &mut indicates the argument is a reference
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

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
