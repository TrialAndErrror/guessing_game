

use std::cmp::Ordering;
use rand::Rng;
use std::io;


fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("\n\n\n\n\n\n\nTry to guess the number I am thinking of, from 1 to 100");

    loop {
        println!("Enter your guess!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Error reading line");

        println!("You guessed {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(..) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("Correct! You Win!");
                break; 
            }
 
        }
    }
}
