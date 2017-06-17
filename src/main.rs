extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guessing game project!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number {}", secret_number);

    loop {
        println!("State your guess!!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read guess...");

        let guess: u32 = match guess.trim().parse(){
            Err(_) => continue,
            Ok(num) => num,
        };
        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
