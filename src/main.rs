use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let a = 10;
    let b:i32= 20;
    let c = 30i32;
    let d = 30_i32;
    let e = add(add(a,b),add(c,d));
    println!("e is {}",e);
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32= match guess.trim().parse() {
            Ok(num) => num,
            Err(_)=>continue,
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}

fn add(i:i32,j:i32)->i32{
    i+j
}