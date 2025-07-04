use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number !!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Please imput your guess: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("To small"),
            Ordering::Greater => println!("You win"),
            Ordering::Equal => {
                println!("You win !!");
                break;
            }
        }
    }
}
