use std::io;


pub fn main() {
    // program to find whether given number is prime or not
    println!("Prime number checker");
    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Hey enter something proper ya dummy");
    let num: i32 = inp.trim().parse::<i32>().expect("Not a number, buddy");
    
    let mut is_prime = true;
    let mut i = 2;
    while i*i <= num {
        if num % i == 0 {
            is_prime = false;
            break;
        }
        i += 1;
    }
    if is_prime {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }
}