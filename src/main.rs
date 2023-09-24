use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let random_number = rand::thread_rng().gen_range(1..=100);

    // println!("random number is {random_number}");
    loop {
        println!("Input your guess");
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = guess.trim().parse().expect("Please type a number");
        
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
