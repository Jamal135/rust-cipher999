#![allow(dead_code)] // Temporary
use rand::distributions::uniform::SampleUniform;
use rand::distributions::Uniform;
use rand::Rng;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fs;
use std::io;
use std::str::FromStr;

// Creation Date: 10/09/2022

// Load file at file path into u8 vector of bytes
fn load_file_bytes(file_path: &str) -> Vec<u8> {
    let contents = fs::read_to_string(file_path).expect("Unable to read file");
    return contents.into_bytes();
}

// Prints message argument and returns user input
fn user_argument(message: &str) -> String {
    println!("{message}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    return input;
}

// Get and verify user input as specified integer type
fn get_number<T: Display + PartialOrd + SampleUniform + FromStr>() -> T
where
    <T as FromStr>::Err: Display,
{
    let message: &str = "Enter integer:";
    let input = loop {
        match user_argument(message).trim().parse::<T>() {
            Ok(input) => break input,
            Err(e) => println!("Enter valid number!\n{e}\n"),
        }
    };
    return input;
}

// Get random number between min and max as specified integer type
fn random_number<T: Display + PartialOrd + SampleUniform>(min: T, max: T) -> T {
    let random_number: T = rand::thread_rng().gen_range(min..max);
    return random_number;
}

// Get random numbers vector between min and max of length as specified integer
fn random_vector<T: Display + PartialOrd + SampleUniform>(min: T, max: T, length: u32) -> Vec<T> {
    let range = Uniform::from(min..max);
    let mut random = rand::thread_rng();
    return (0..length).map(|_| random.sample(&range)).collect();
}

fn main() {
    let bytes = load_file_bytes("test.txt");
    println!("{:?}", bytes);
    let number = get_number::<u32>();
    println!("{number}");
}
