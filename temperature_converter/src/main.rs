use std::io;
use std::process;

fn main() {
    println!("Welcome to the converter of temperature!");
    loop{
        println!("'C' for converting from Celsius to Fahrenheit or 'F' for reverse operation or break to end");
        let mut converter = String::new();
        io::stdin().read_line(&mut converter).expect("Expected 'C' or 'F'");
        if converter == "break\n" {
            process::exit(1);
        }
        if converter != "C\n" && converter != "F\n" {
            println!("Enter 'C' or 'F'!");
            continue;
        }
        println!("Input temperature to convert!");

        let mut temperature = String::new();
        io::stdin().
                read_line(&mut temperature).
                expect("Expected integer for temperature");
        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if converter == "C\n"{
            let fahrenheit_temperature = temperature * 1.8 + 32 as f32;
            println!("{} in Celsius is {} in Fahrenheit", temperature, fahrenheit_temperature); 
        }
        else if converter == "F\n"{
            let celsius_temperature = (temperature - 32 as f32) * 5 as f32 / 9 as f32;
            println!("{} in Fahrenheit is {} in Celsius", temperature, celsius_temperature); 
        }
    }
}

