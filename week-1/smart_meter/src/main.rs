use std::io;

fn main(){
    println!{"EKEDC Smart Meter"};
    println!{"Input your electricity usage for this month:"}

    let mut usage = String::new();
    io::stdin().read_line(&mut usage).expect("Invalid Input");

    let usage: f32 = usage.trim().parse().expect("Enter a value");

    let bill: f32;

    if (100000.0..=200000.0).contains(&usage) {
        println!("Charge per unit: ₦25");
        bill = usage * 25.0;
        println!("Total Electricity Bill: ₦{bill}");
    } else if usage > 200000.0 {
        println!("Charge per unit: ₦30");
        bill = usage * 30.0;
        println!("Total Electricity Bill: ₦{bill}");
    } else{
        println!("Charge per unit: ₦20");
        bill = usage * 20.0;
        println!("Total Electricity Bill: ₦{bill}");
    }
}
