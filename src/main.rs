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

fn shift_string(string_to_shift: &str, string_key: &str) -> String{
    let pair: Vec<(char,char)> = string_to_shift.chars()
    .zip(string_key.chars())
    .collect();

    pair.iter()
    .map(|(c1,c2)|shift_character(*c1,*c2))
    .collect::<String>()
}

fn get_best_fitness(text: &str, group_size: u32, frequency_chart: HashMap<String, u32>) -> String{
    let mut best_fitness = vec![(String::new(),0); group_size as usize];
    for i in 0..group_size as usize{
        for first_char_key in 0..26u8{
            for second_char_key in 0..26u8{
                let key: String = vec![(first_char_key+97) as char,(second_char_key+97) as char].iter().collect();
                let mut fitness = (key.clone(),0);
                for text_idx in (i..(text.len()-1)).step_by(group_size as usize){
                    let mut bigram_to_count = String::new();
                    bigram_to_count.push(text.chars().nth(text_idx.into()).unwrap());
                    bigram_to_count.push(text.chars().nth((text_idx+1).into()).unwrap());
                    let shifted_bigram = shift_string(&bigram_to_count, &key);
                    fitness.1 += frequency_chart.get(&shifted_bigram).unwrap();
                }
                if best_fitness[i].1 < fitness.1 {
                    best_fitness[i] = fitness;
                }
            }
        }
    }
    let mut key = String::new();
    let mut fitness_before = 0;
    for i in 0..best_fitness.len() {
        if best_fitness[i].1 > fitness_before {
            key.push_str(best_fitness[i].0.as_str());
        }
        fitness_before = best_fitness[i].1;
    }
    key[..group_size as usize].to_string()
}
fn solve(text: &str, frequency_chart: HashMap<String, u32>) -> String{
    // let groups = make_groups(text, 5);
    let mut text_fitness = 0;
    let mut best_solution = String::new();
    for key_size in 2..6{
        let key = get_best_fitness(text, key_size, frequency_chart.clone());
        let possible_solution = _decipher(text, key.as_str());
        let mut fitness = 0;
        for i in 0..(possible_solution.len()-1){
            let char1 = possible_solution.chars().nth(i).unwrap();
            let char2 = possible_solution.chars().nth(i+1).unwrap();
            let bigram: String = vec![char1, char2].iter().collect();
            fitness +=  frequency_chart.get(&bigram).unwrap_or(&0);
        }
        if fitness > text_fitness {
            text_fitness = fitness;
            best_solution = possible_solution;
        }
    } 
    return best_solution;
}

fn challenge(frequency_chart: HashMap<String, u32>, text: &str)-> String{
    solve(text, frequency_chart)
}

fn main() {
    let mut args = env::args();
    // let mut path = args.nth(1).unwrap();
    let path = "/home/caio/vigenere-cipher/src/german_bigrams.txt".to_string();
    let mut _letter_freq = fs::read_to_string(path).unwrap();
    let letter_freq:HashMap<String, u32> = _letter_freq
    .split('\n')
    .map(|line| {
        let mut tokens = line.split(' ');
        let (letters, value) = (tokens.next().unwrap().to_lowercase(), tokens.next().unwrap());
        
        (letters, value.parse::<u32>().unwrap())
    }).collect();
    // let path_challenge = args.nth(0).unwrap();
    let path_challenge = "/home/caio/vigenere-cipher/src/challenge.txt";
    let _challenge = fs::read_to_string(path_challenge).unwrap();
    
    let _result = challenge(letter_freq, _challenge.as_str());
    println!("{:?}", _result);
    // let ciphred_text = cipher("testandoessetrabalhoincrivel", "teste");
    // let deciphred_text = decipher(desafio, "arara");
    // println!("{}", deciphred_text);
}