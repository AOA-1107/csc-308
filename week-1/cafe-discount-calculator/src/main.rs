 use std::io;

fn main(){
    println!("Cafe Discount Calculator");
    println!("Enter bill:");


let mut bill = String::new();
io::stdin().read_line(&mut bill).expect("Invalid input");

let amount: f32 = bill.trim().parse().expect("Enter a value");

// let final_amount: i32 

if (5001.0..=10000.0).contains(&amount) {
    println!("Original Bill: ₦{amount}");
    println!("Discount Applied: 10%");
    let amount = amount - (amount * (10.0 / 100.0));
    println!("Final Bill: ₦{amount}");
} else if amount > 10000.0 {
    println!("Original Bill: ₦{amount}");
    println!("Discount Applied: 15%");
    let amount = amount - (amount * (15.0 / 100.0));
    println!("Final Bill: ₦{amount}");
}else {
    println!("No applicable discount")
}
}