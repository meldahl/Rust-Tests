use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn random(min: i8, max: i8) -> i8 {
    return rand::thread_rng().gen_range(min..=max);
}

fn main() {
    println!("Guess the number!");

    let random_number = random(1, 100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease only input a number between 1 and 100!\n");
                continue;
            }
        };

        if guess < 1 || guess > 100 {
            println!("\nPlease only input a number between 1 and 100!\n");
            continue;
        }

        println!("You guessed: {guess}");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
