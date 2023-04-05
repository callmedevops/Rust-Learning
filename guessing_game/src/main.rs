use std::cmp::Ordering; // remember this as to compare the number by Ordering methods
use rand::Rng; // don't confuse with range crates 
use std::io; // for standard input

fn main() {

let secret_number:i8= rand::thread_rng().gen_range(1..=10); // make sure you don't confuse the thread_rang() and gen_range()

for i in 1..4 {
    println!("This is your {} attempt ", i);
    let mut your_input = String::new(); // this is fro decalring the string for future use 
    io::stdin().read_line(&mut your_input).expect("Failed to read string"); // we then use the above input variable to be filled by the run time
    let your_input:i8 =   match your_input.trim().parse() {     // 1) this math is for restricting the object enum value of the input match with the num type
        Ok(num) => num,                                         // 2) also used to convert the input string into the integer value
        Err(_) => continue,
        };
    match your_input.cmp(&secret_number){                       // we can we the old school if else method but we'll be using the rust cmp standard library which then order them and compare them 
       Ordering::Equal => {
        println!("You win!");
        break;
        }
       Ordering::Greater => println!("You are too high!"),
       Ordering::Less => println!("You are too less")
    }
}
}