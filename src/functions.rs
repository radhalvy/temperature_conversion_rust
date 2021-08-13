use std::io;

// Gets user answer about conversion type
pub fn conversion_type() -> u8 {
    
    let mut user_answer = String::new();

    println!("What do you want to convert to:\nCelsius to Fahrenheit (1) \tFahrenheit to Celsius (2) \tCelsius to Kelvin? (3)");
    io::stdin()
        .read_line(&mut user_answer)
        .expect("Failed to read line");

    let user_answer: u8 = user_answer
        .trim()
        .parse()
        .expect("Invalid input");

    user_answer
}

// Gets celsius temperature from user
pub fn user_celsius_value() -> f64 {
    
    let mut user_celsius = String::new();
    
    println!("\nEnter the temperature in celsius:");
    io::stdin()
        .read_line(&mut user_celsius)
        .expect("Failed to read line");

    let user_celsius: f64 = user_celsius
        .trim()
        .parse()
        .expect("Invalid input");

    user_celsius
}

// Gets fahrenheit temperature from user
pub fn user_fahrenheit_value() -> f64 {

    let mut user_fahrenheit = String::new();

    println!("\nEnter the temperature in fahrenheit:");
    io::stdin()
        .read_line(&mut user_fahrenheit)
        .expect("Failed to read line");

    let user_fahrenheit: f64 = user_fahrenheit
        .trim()
        .parse()
        .expect("Invalid input");

    user_fahrenheit
}
