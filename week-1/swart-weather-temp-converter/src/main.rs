use std::io; 

fn main () {
    println!("Smart Weather Temperature Converter");
    println!("Choose an option (1 or 2): ");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Invalid input");

    let choice = choice.trim();

    if choice == "1" {
        println!(" Enter temperature in Celsius: ");
        let mut celsius = String::new();
        io::stdin().read_line(&mut celsius).expect(" Invalid input");
        let celsius: f32 = celsius.trim().parse().expect(" Enter a value");
        let fahrenheit = ( celsius * 9.0 / 5.0 ) + 32.0;
        println!("{celsius}°C = {fahrenheit}°F");
    } else if choice == "2" {
        println!(" Enter temperature in Fahrenheit: ");
        let mut fahrenheit = String::new();
        io::stdin().read_line(&mut fahrenheit).expect(" Invalid input");
        let fahrenheit: f32 = fahrenheit.trim().parse().expect(" Enter a value");
        let celsius = ( fahrenheit - 32.0) * 5.0 /9.0 ;
        println!("{fahrenheit}°F = {celsius}°C");
    } else { 
        println!("Invalid choice");
    }
}