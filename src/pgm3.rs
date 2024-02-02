use std::io::stdin;

pub fn main(){
    let mut inp = String::new();
    println!("Enter a limit: ");
    stdin().read_line(&mut inp).expect("Hey enter something proper ya dummy");
    let num: i32 = inp.trim().parse().expect("Not a number...");
    let mut x:i32 = 0;
    let mut y:i32 = 1;
    for _ in 0..num {
        print!("{}, ", x);
        let z = x + y;
        x = y;
        y = z;
    }
}