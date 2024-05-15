use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Can you guess the secret number ?");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("please input your guess");
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = guess.trim().parse().expect("type should be a number");

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
