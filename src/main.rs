use std::{vec, collections::HashMap,fs, env};
use regex::Regex;
const alphabet:&str = "abcdefghijklmnopqrstuvwxyz";
fn _cipher(text: &str, key: &str) -> String {
    let mut ciphred_text = String::from("");
    for (i,letter) in text.chars().enumerate() {
        // println!("{}", letter as u32-97);
        let letter_value = letter as u32-97;
        let key_letter = key.as_bytes()[i%key.len()] as u32-97;
        let new_index = (letter_value+key_letter)%(alphabet.len() as u32);
        let ciphred_letter = alphabet.chars().nth(new_index.try_into().unwrap()).unwrap();
        ciphred_text.push(ciphred_letter);
    }
    return ciphred_text;
}

fn _decipher(text: &str, key: &str) -> String {
    let mut deciphred_text = String::from("");
    let mut counter_non_chars = 0;
    for (i, letter) in text.chars().enumerate() {
        if ![' ', '\n', ';', '\'', '—','-',',','.'].contains(&letter){
            let index = i-counter_non_chars;
            let let_value = letter as i32-97;
            let key_let = key.as_bytes()[index%key.len()] as i32-97;
            let new_index = (let_value-key_let + (alphabet.len()) as i32) % (alphabet.len()) as i32;
            
            let deciphred_letter = alphabet.chars().nth(new_index.try_into().unwrap()).unwrap();
            deciphred_text.push(deciphred_letter);
        } else {
            deciphred_text.push(letter);
            counter_non_chars+=1;
        }
    }
    return deciphred_text;
}

fn shift_character(char_to_shift: char, char_key: char) -> char{
    let key_value = char_key as i32 - 97;
    let char_value = char_to_shift as i32 - 97;
    let new_index = (char_value-key_value + (alphabet.len()) as i32) % (alphabet.len()) as i32;
    let shiftted_char = alphabet.chars().nth(new_index.try_into().unwrap()).unwrap();
    shiftted_char
}

fn make_groups(text: &str, module: u32)->Vec<String>{
    let groups = text.chars()
    .collect::<Vec<char>>()
    .chunks(module as usize)
    .map(|c| c.iter().collect::<String>())
    .collect::<Vec<String>>();
    
    groups
}

fn get_best_fitness(text: Vec<String>, group_size: usize, frequency_chart: HashMap<String, u32>){
    for i in 0..=group_size{
        let mut bigrams_to_analise = vec![String::new(); group_size];
        // Add the bigram to the vector to futher analise
        for (i, group) in text.iter().enumerate() {
            let mut chars_iter = group.chars();
            bigrams_to_analise[i].push(chars_iter.nth(0).unwrap());
            bigrams_to_analise[i].push(chars_iter.nth(0).unwrap());
        }
        //check each possible bigram (656 combinations)
        let best_fitness = vec![0; group_size];
        for i in 0..26u8{
            for j in 0..26u8{
                let sum = 0;
                // create bigram from iterators
                let mut bigram_key = String::new();
                bigram_key.push(i as char);
                bigram_key.push(j as char);
                
                
                
            }
        }
    } 
    return
}
fn solve(text: &str, key_size: u32, frequency_chart: HashMap<String, u32>){
    let groups = make_groups(text, 5);
    let key = get_best_fitness(groups, 5, frequency_chart);
    println!("{:?}", groups);
}

fn challenge(frequency_chart: HashMap<String, u32>, text: &str){
    solve(text, 5)
}

fn main() {
    let mut args = env::args();
    // let mut path = args.nth(1).unwrap();
    let path = "/home/caio/unb/sc/vigenere-cipher/src/english_bigrams.txt".to_string();
    let mut _letter_freq = fs::read_to_string(path).unwrap();
    let letter_freq:HashMap<String, u32> = _letter_freq
    .split('\n')
    .map(|line| {
        let mut tokens = line.split(' ');
        let (letters, value) = (tokens.next().unwrap().to_lowercase(), tokens.next().unwrap());
        
        (letters, value.parse::<u32>().unwrap())
    }).collect();
    // let path_challenge = args.nth(0).unwrap();
    let path_challenge = "/home/caio/unb/sc/vigenere-cipher/src/challenge.txt";
    let _challenge = fs::read_to_string(path_challenge).unwrap();
    
    let _result = challenge(letter_freq, _challenge.as_str());
    // let ciphred_text = cipher("testandoessetrabalhoincrivel", "teste");
    // let deciphred_text = decipher(desafio, "arara");
    // println!("{}", deciphred_text);
}