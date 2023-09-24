use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Input your guess");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        match guess.trim().eq(&String::from("q")) {
            true => {
                println!("Thanks For Playing !");
                break
            }
            false => (),
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue
            }
        };
        
        println!("You've guessed {guess}");
    
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big !"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
        }
    }
}
