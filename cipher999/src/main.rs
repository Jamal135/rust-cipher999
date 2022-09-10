use rand::Rng;
use std::io;

fn user_argument(message: &str) -> String {
    println!("{message}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input;
}

fn get_number() -> u32 {
    let message: &str = "Enter integer:";
    let input = loop {
        match user_argument(message).trim().parse::<u32>() {
            Ok(input) => break input,
            Err(e) => println!("Enter valid number!\n{e}\n"),
        }
    };
    return input;
}

fn print_random_number() {
    let number: u32 = rand::thread_rng().gen_range(u32::MIN..u32::MAX);
    println!("{number}")
}

fn main() {
    loop {
        let n: u32 = get_number();
        for _ in 0..n {
            print_random_number();
        }
        println!("\n")
    }
}
