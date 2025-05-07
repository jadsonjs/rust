use std::io;  // To obtain user input and then print the result as output, we need to bring the io input/output library into scope.
use rand::Rng; // The Rng trait defines methods that random number generators implement
use std::cmp::Ordering; // The cmp module defines the ordering of values. It’s used to compare two values and determine their relative order.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    loop{ // create an infinite loop

        println!("The secret number is: {secret_number}");

        println!("Please input your guess.");

        // Next, we’ll create a variable to store the user input, like this
        // In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change
        // The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String.
        let mut guess = String::new();  

        // Next, the line .read_line(&mut guess) calls the read_line method on the standard input handle to get input from the user. 
        // We’re also passing &mut guess as the argument to read_line to tell it what string to store the user input in.
        // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        
        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess, for example.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {}", guess);

    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // break the loop
            }
        }
    }
}
