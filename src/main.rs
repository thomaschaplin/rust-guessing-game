use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn print_guess_the_number_message(min: u32, max: u32) {
    println!("\nGuess the number between {} & {}\n", min, max);
}

fn print_input_your_guess_message() {
    println!("Please input your guess.\n");
}

fn print_winning_message(answer: u32) {
    println!("\n{} is correct, you win!\n", answer);
}

fn print_please_input_your_number_message(option: &str) {
    println!("\nPlease input your {} number.\n", option);
}

fn print_too_small_message(guess: u32) {
    println!("\n{} is too small, try again!\n", guess);
}

fn print_too_large_message(guess: u32) {
    println!("\n{} is too large, try again!\n", guess);
}

fn panic_when_maximum_is_smaller_than_minimum(max: u32, min: u32) {
    panic!("{} is smaller than {}, pick a larger number!\n", max, min);
}

fn generate_random_number_between(min: u32, max: u32) -> u32 {
    rand::thread_rng().gen_range(min..max)
}

fn parse_guess_as_u32(guess: String) -> u32 {
    let guess_as_u32: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(
            "Failed to read value \"{}\", we were expecting a number!",
            guess.trim()
        ),
    };

    guess_as_u32
}

fn get_user_defined_input_as_u32() -> u32 {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    

    parse_guess_as_u32(guess)
}

fn get_user_defined_input(option: &str) -> u32 {
    print_please_input_your_number_message(option);

    get_user_defined_input_as_u32()
}

fn get_minimum_and_maximum_inputs() -> [u32; 2] {
    let min = get_user_defined_input("minimum");
    let max = get_user_defined_input("maximum");

    [min, max]
}

fn validate_maximum_input(min: u32, max: u32) {
    match min.cmp(&max) {
        Ordering::Less => {
            game(min, max);
        }
        Ordering::Greater => panic_when_maximum_is_smaller_than_minimum(max, min),
        Ordering::Equal => {
            game(min, max);
        }
    }
}

fn does_guess_equal_answer_loop(answer: u32) {
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&answer) {
            Ordering::Less => print_too_small_message(guess),
            Ordering::Greater => print_too_large_message(guess),
            Ordering::Equal => {
                print_winning_message(answer);
                break;
            }
        }
    }
}

fn game(min: u32, max: u32) {
    print_guess_the_number_message(min, max);
    let random_number = generate_random_number_between(min, max);
    print_input_your_guess_message();
    does_guess_equal_answer_loop(random_number);
}

fn main() {
    let [min, max] = get_minimum_and_maximum_inputs();
    validate_maximum_input(min, max)
}
