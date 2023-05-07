use std::{collections::HashMap,fs, io::Write};

use clap::{Parser,ValueEnum};

mod bigram_counter;
mod decipher;

/// Program to cipher, decipher and break a vigenere cipher
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// What mode to run the program in
    #[arg(value_enum)]
    mode: Mode,

    ///Key size
    #[arg(short)]
    key_size: Option<u32>,
    arg0: Option<String>,
    arg1: Option<String>,
    arg2: Option<String>,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// Decipher the text on arg0 path and outputs the result inside arg1 path
    Decipher,
    /// Cipher the text on arg0 path and outputs the result inside arg1 path
    Cipher,
    /// Solve the cipher on arg0 path using the logs inside arg1 path and outputs the result inside arg2 path
    Solver,
    /// Calculate the bigrams log probability in arg0 path and outputs inside arg1 path
    Processer
}

fn cipher_text(args: &Cli){
    let mut pw = String::new();
    println!("Enter the password:");
    let text = fs::read_to_string(args.arg0.as_ref().unwrap()).unwrap();
    std::io::stdin().read_line(&mut pw).unwrap();
    pw.pop();
    let result = decipher::cipher(&text, &pw);
    let file = fs::File::create(args.arg1.as_ref().unwrap_or(&"./out.txt".to_string()));
    match file {
        Ok(mut f) => f.write_all(result.as_bytes()).ok().unwrap(),
        Err(e) => panic!("{}",e) 
    }
}

fn decipher_text(args: &Cli){
    let mut pw = String::new();
    println!("Enter the password:");
    let text = fs::read_to_string(args.arg0.as_ref().unwrap()).unwrap();
    std::io::stdin().read_line(&mut pw).unwrap();
    pw.pop();
    let result = decipher::decipher(&text, &pw);
    let file = fs::File::create(args.arg1.as_ref().unwrap_or(&"./out.txt".to_string()));
    match file {
        Ok(mut f) => f.write_all(result.as_bytes()).ok().unwrap(),
        Err(e) => panic!("{}",e) 
    }
}

fn processes_text(args: &Cli) {
    let output_path = args.arg1.as_ref().unwrap();
    let text_path = args.arg0.as_ref().unwrap();

    let _res = bigram_counter::count_bigrams(text_path, output_path).unwrap();
}

fn challenge(args: &Cli){
    let mut _letter_freq = fs::read_to_string(args.arg0.clone().unwrap()).unwrap();
    let letter_freq:HashMap<Vec<u8>, u32> = _letter_freq
    
    .split('\n')
    .map(|line| {
        let mut tokens = line.split(' ');
        let (letters, value) = (tokens.next().unwrap().to_lowercase().bytes().collect(), tokens.next().unwrap());
        
        (letters, value.parse::<u32>().unwrap())
    }).collect();
    
    let _challenge = fs::read_to_string(args.arg1.clone().unwrap()).unwrap();
    // let re = Regex::new(r"[^a-z]").unwrap();
    // _challenge = re.replace_all(_challenge.as_str(), "").to_string();
    let mut text: String = String::new();
    for letter in _challenge.as_bytes(){
        match letter {
            b'a'..=b'z'=>text.push(*letter as char),
            b'A'..=b'Z'=> text.push((letter+32) as char),
            _=>{}
        };
    }
    let result = decipher::solve(text.as_str(), letter_freq, args.key_size.unwrap_or(30) as u64);
    let file = fs::File::create(args.arg2.as_ref().unwrap_or(&"./out.txt".to_string()));
    match file {
        Ok(mut f) => f.write_all(result.as_bytes()).ok().unwrap(),
        Err(e) => panic!("{}",e) 
    }
}

fn main() {
    let args = Cli::parse();
    match args.mode{
        Mode::Processer => processes_text(&args),
        Mode::Solver => challenge(&args),
        Mode::Cipher => cipher_text(&args),
        Mode::Decipher => decipher_text(&args)
    }
}