use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn try_again() -> &'static str {
    return "try again";
}

fn you_win(num: u32) {
    println!("{} is correct, you win!", num);
}

fn generate_random_number_between(min: u32, max: u32) -> u32 {
    return rand::thread_rng().gen_range(min, max);
}

fn main() {
    println!("Guess the number between 1 & 100\n");

    let secret_number = generate_random_number_between(1, 101);

    println!("Please input your guess.\n");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{} is too small, {}!", &guess, try_again());
                try_again();
            }
            Ordering::Greater => {
                println!("{} is too large, {}!", &guess, try_again());
                try_again();
            }
            Ordering::Equal => {
                you_win(secret_number);
                break;
            }
        }
    }
}
