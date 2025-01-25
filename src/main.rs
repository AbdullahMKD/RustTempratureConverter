use std::io::{self};

fn main() {

    println!("Temprature Converter");
    println!("Input q to quit the program");

    loop {
        let deg = read_input("Please input temperature scale C for Celsius, F for Fahrenheit, or q to quit: ");

        if deg == "c" || deg == "f" {

            let temp = read_input("Please input the temperature you want to convert: ");

            let temp: f32 = match temp.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Failed to convert string to number, Please try again"); 
                    continue;
                }
            };
            if deg == "c" {
                let result = celsiusfahrenheit(temp);
                println!("The Converted value is {} Fahrenheit", result);
            } else if deg == "f" {
                let result = fahrenheitcelsius(temp);
                println!("The Converted value is {} Celsius", result);
            }
        } else if deg == "q" {
            break;
        } else {
            println!("Please input a valid character");
        }
    }

    println!("Thank you")
    
}

fn celsiusfahrenheit(cel: f32) -> f32 {
    (cel * (1.8)) + 32.0
}

fn fahrenheitcelsius(fah: f32) -> f32 {
    (fah - 32.0) / (1.8)
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_lowercase()
}

