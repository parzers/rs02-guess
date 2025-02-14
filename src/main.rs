use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");
    
    loop {
        println!("Please input your guess.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        
        let guess: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(what) => {
                println!("Failed to parse number: {}", what);
                continue;
            },
        };
        
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("That is too low!"),
            std::cmp::Ordering::Equal => { 
                println!("That is correct!");
                break;
            },
            std::cmp::Ordering::Greater => println!("That is too high!"),
        }
    }
    println!("The secret number was: {}", secret_number);
}
