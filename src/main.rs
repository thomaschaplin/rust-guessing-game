use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn you_win(num: u32) {
    println!("{} is correct, you win!", num);
}

fn generate_random_number_between(min: u32, max: u32) -> u32 {
    return rand::thread_rng().gen_range(min, max);
}

fn game(min: u32, max: u32) {
    println!("Guess the number between {} & {}\n", min, max);

    let secret_number = generate_random_number_between(min, max);

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
                println!("{} is too small, try again!", &guess);
            }
            Ordering::Greater => {
                println!("{} is too large, try again!", &guess);
            }
            Ordering::Equal => {
                you_win(secret_number);
                break;
            }
        }
    }
}

fn main() {
    game(1, 100)
}
