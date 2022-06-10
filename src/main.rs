use rand::Rng;
use std::cmp::Ordering;
use std::io;
// $env:https_proxy="http://proxy29.ad.campus-eni.fr:3128"

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    println!("Guess the number");
    println!();

    loop {
        println!("Input your guess: ");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number");

        println!("You guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => println!("You win"),
            Ordering::Greater => println!("Too big"),
        }
    }
}
