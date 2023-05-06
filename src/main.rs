use std::{collections::HashMap,fs};
use regex::Regex;

use clap::Parser;

mod bigram_counter;
mod decipher;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file containing the bigrams probability in log
    #[arg(short, long)]
    bigrams_path: String,

    /// Path to the file containing the cipher to decipher without knowing the key
    #[arg(short, long)]
    cipher_path: String,
}

fn challenge(frequency_chart: HashMap<Vec<u8>, u32>, text: &str)-> String{
    decipher::solve(text, frequency_chart)
}

fn main() {
    let args = Args::parse();
    let mut _letter_freq = fs::read_to_string(args.bigrams_path).unwrap();
    let letter_freq:HashMap<Vec<u8>, u32> = _letter_freq

    .split('\n')
    .map(|line| {
        let mut tokens = line.split(' ');
        let (letters, value) = (tokens.next().unwrap().to_lowercase().bytes().collect(), tokens.next().unwrap());
        
        (letters, value.parse::<u32>().unwrap())
    }).collect();

    let mut _challenge = fs::read_to_string(args.cipher_path).unwrap();
    let re = Regex::new(r"[^a-z]").unwrap();
    _challenge = re.replace_all(_challenge.as_str(), "").to_string();
    let _result = challenge(letter_freq, _challenge.as_str());
    println!("{}", _result);
}