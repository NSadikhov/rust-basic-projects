use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    guess();
}

fn guess() {
    println!("< Guess the number >");

    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..101);

    loop {
        println!("< Input Your Guess > : ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("< Too Small >"),
            Ordering::Greater => println!("< Too Big >"),
            Ordering::Equal => {
                println!("< You win >");
                break;
            }
        }
    }
}
