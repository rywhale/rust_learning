use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Input your guess.");

        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        // Check that guess is a number
        // u32 can only contain numerical characters
        // .parse() requires that we tell it which type of number guess is
        let guess: u32 = match guess.trim().parse() {
            // If parsing works, return number
            Ok(num) => num,
            // Otherwise skip loop iteration
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
