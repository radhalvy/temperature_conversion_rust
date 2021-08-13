mod functions;

fn main() {

    match functions::conversion_type() {
        
        // If true, makes the conversion from celsius to fahrenheit
        1 => {
            let final_conversion = functions::user_celsius_value() * 1.8 + 32 as f64;
            println!("\nThe temperature in fahrenheit is {:.2}\n", final_conversion);
        }
        // If true, makes the conversion from fahrenheit to celsius
        2 => {
            let final_conversion = (functions::user_fahrenheit_value() - 32 as f64) / 1.8;
            println!("\nThe temperature in celsius is {:.2}", final_conversion);
        }
        3 => {
            let final_conversion = functions::user_celsius_value() + 273.15;
            println!("\nThe temperature in kelvin is {:.2}", final_conversion);
        }
        _ => {
            println!("Invalid option");
        }
    };
    
}
