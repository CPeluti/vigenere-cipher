use std::{vec, collections::HashMap};

use rayon::prelude::*;
use indicatif::ProgressBar;

const ALPHABET:&str = "abcdefghijklmnopqrstuvwxyz";

pub fn cipher(text: &str, key: &str) -> String {
    let mut ciphred_text = String::from("");
    let text = text.as_bytes();
    let mut counter = 0;
    for letter in text {
        // println!("{}", letter as u32-97);
        match letter {
            b'a'..=b'z' => {
                let letter_value = letter-97;
                // testetesteteste
                let key_letter = key.as_bytes()[counter%(key.len())]-97;
                counter+=1;
                let ciphred_letter = ((letter_value+key_letter)%(ALPHABET.len() as u8)+97) as char;
                ciphred_text.push(ciphred_letter);
            }
            b'A'..=b'Z' => {
                let letter_value = letter+32-97;
                // testetesteteste
                let key_letter = key.as_bytes()[counter%(key.len())]-97;
                counter+=1;
                let ciphred_letter = ((letter_value+key_letter)%(ALPHABET.len() as u8)+97-32) as char;
                ciphred_text.push(ciphred_letter);
            }
            _=>{ciphred_text.push(*letter as char)}
        }
    }
    ciphred_text
}

fn shift_character(char_to_shift: u8, char_key: u8) -> u8{
    let key_value = char_key - 97;
    let char_value = char_to_shift - 97;
    let new_index = (char_value-key_value + (ALPHABET.len()) as u8) % (ALPHABET.len()) as u8;
    new_index + 97
}

fn shift_string(string_to_shift: &[u8], string_key: &[u8]) -> Vec<u8>{
    string_to_shift.iter()
    .zip(string_key)
    .map(|(c1,c2)|shift_character(*c1,*c2))
    .collect()
}

fn get_best_fitness(text: &str, group_size: u32, frequency_chart: HashMap<Vec<u8>, u32>) -> String{
    let mut best_fitness = vec![(Vec::<u8>::new(),0); group_size as usize];
    let text_bytes = text.as_bytes();
    for (i, best_fitness_idx) in best_fitness.iter_mut().enumerate(){ // key_size
        for first_char_key in 0..26u8{
            for second_char_key in 0..26u8{
                // Use key as bytes to futher optimize
                let key: Vec<u8> = vec![(first_char_key+97),(second_char_key+97)];
                let mut fitness = 0;
                for text_idx in (i..(text.len()-1)).step_by(group_size as usize){

                    let bigram_to_count = vec![text_bytes[text_idx], text_bytes[text_idx+1]];

                    let shifted_bigram = shift_string(&bigram_to_count, &key);

                    fitness += frequency_chart.get(&shifted_bigram).unwrap();
                }
                if best_fitness_idx.1 < fitness {
                    *best_fitness_idx = (key, fitness);
                }
            }
        }
    }
    let mut key = String::new();
    let mut fitness_before = 0;
    for i in 0..best_fitness.len() {
        if best_fitness[i].1 > fitness_before {
            key.push(best_fitness[i].0[0] as char);
        } else {
            key.push(best_fitness[i-1].0[1] as char)
        }
        fitness_before = best_fitness[i].1;
    }
    key[..group_size as usize].to_string()
}

pub fn decipher(text: &str, key: &str) -> String {
    let mut deciphred_text = String::from("");
    let mut counter_non_chars = 0;
    for (i, letter) in text.chars().enumerate() {
        if ![' ', '\n', ';', '\'', '—','-',',','.'].contains(&letter){
            let index = i-counter_non_chars;
            let let_value = letter as i32-97;
            let key_let = key.as_bytes()[index%(key.len())] as i32-97;
            let new_index = (let_value-key_let + (ALPHABET.len()) as i32) % (ALPHABET.len()) as i32;
            
            let deciphred_letter = ALPHABET.chars().nth(new_index.try_into().unwrap()).unwrap();
            deciphred_text.push(deciphred_letter);
        } else {
            deciphred_text.push(letter);
            counter_non_chars+=1;
        }
    }
    deciphred_text
}

pub fn solve(text: &str, frequency_chart: HashMap<Vec<u8>, u32>, key_size:u64) -> String{

    let max_key_size = key_size;
    println!("Deciphering the text!");
    let pb = ProgressBar::new(max_key_size);
    //Bruteforce each key
    let max = (2..=max_key_size as u32).into_par_iter().map(|key_size|{

        let key = get_best_fitness(text, key_size, frequency_chart.clone());
        // Maybe should change the output of decipher
        let possible_solution: Vec<u8> = decipher(text, key.as_str()).bytes().collect();
        let mut fitness = 0;
        
        for i in 0..(possible_solution.len()-1){
            let bigram = &possible_solution[i..=i+1];
            fitness +=  frequency_chart.get(bigram).unwrap_or(&0);
        }
        pb.inc(1);
        (fitness,possible_solution, key)
    }).max_by_key(|a| a.0).unwrap();
    pb.finish_with_message("done");
    println!("{}", max.2);
    String::from_utf8(max.1).unwrap_or_default()
}