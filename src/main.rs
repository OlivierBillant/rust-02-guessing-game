use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;
// $env:https_proxy="http://proxy29.ad.campus-eni.fr:3128"

fn main() {
    // RNG using rand lib
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    println!("Guess the number");
    println!();

    loop {
        println!("Input your guess: ");
        let mut guess: String = String::new();
        // use stdin read_line to capture the user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // The string needs to be parsed as an int
        // Except handles the error message
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);
        // to compare use std::cmp::Ordering lib
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too big".red()),
            // break allows to exit the loop
            Ordering::Equal => {println!("{}", "You win!".green());break;}
            // Add text coloring using colored lib.
        }
    }
}
