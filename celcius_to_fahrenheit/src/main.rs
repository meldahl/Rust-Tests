use std::io;
use std::io::Write;

fn main() {
    print!("Convert From Celsius to Fahrenheit: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    get_input(&mut input);

    let celcius = match input.trim().parse() {
        Ok(num) => UserInput::Celsius(num),
        Err(_) => UserInput::Invalid,
    };

    match celcius {
        UserInput::Celsius(celcius) => {
            let fahrenheit = (celcius * 9.0 / 5.0) + 32.0;
            println!("{celcius}° Celsius is equal to {fahrenheit}° Fahrenheit.");
        }

        UserInput::Invalid => println!("Invalid input. Please enter a number."),
    }
}

// Helper functions
fn get_input(buffer: &mut String) {
    io::stdin().read_line(buffer).expect("Failed get input.");
}

// Enums
enum UserInput {
    Celsius(f64),
    Invalid,
}
