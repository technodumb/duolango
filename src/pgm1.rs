pub fn pgm1() {
    println!("Lower Main Triangle");
    for i in 0..10 {
        for __ in 0..=i {
            print!("*");
        }
        println!();
    }

    println!("Upper Main Triangle");
    for i in 0..10{
        for _j in 0..i{
            print!(" ");
        }
        for _j in i..10{
            print!("*");
        }
        println!();
    }

    println!("Upper Cross Triangle");
    for i in 0..10 {
        for _j in 0..(10-i){
            print!("*");
        }
        println!();
    }

    println!("Lower Cross Triangle");
    for i in 0..10 {
        for _j in 0..(10-i-1){
            print!(" ");
        }
        for _j in (10-i)..=10 {
            print!("*");
        }
        println!();
    }
}