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

fn ioc(text: &str) -> f64{
    let mut letter_freq_in_text = vec![0; 26];
    for i in text.chars() {
        letter_freq_in_text[(i as usize)-97] += 1;
    }
    let n = text.len();
    let mut sum = 0;
    for freq in letter_freq_in_text.iter() {
        sum += freq*(freq-1);
    }
    let denominator = n*(n-1);
    return 26.0*sum as f64/denominator as f64;

}

fn calculate_sd(freq: &Vec<(Vec<String>,usize,f64)>) -> f64{
    let mut average = 0.0;
    for (_,_, value) in freq {
        average += value;
    }
    average = average/(freq.len()) as f64;
    let mut sum = 0.0;
    for (_,_, value) in freq {
        sum += f64::powf((value-average) as f64,2.0);
    }
    return f64::sqrt(sum/(freq.len() as f64))
}

fn make_groups(text: &str, min_mod: usize, max_mod: usize) -> Vec<(Vec<String>,usize,f64)>{
    // loop for each module
    let mut ioc_groups = vec![];
    for module in min_mod..=max_mod{
        let mut groups = vec![];
        // Initialize N strings for each group
        for _ in 0..module {
            groups.push(String::new());
        }
        for (index, char) in text.chars().enumerate(){
            groups[index%module].push(char);
        }
        let iocs = groups.iter().map(|s| ioc(s)).collect::<Vec<f64>>();

        let mut groups_ioc_med = iocs.iter().sum();

        groups_ioc_med = groups_ioc_med/iocs.len() as f64;
        ioc_groups.push((groups,module, groups_ioc_med));
    }
    return ioc_groups;
}

fn chi_square(text: &String, letter_freq: HashMap<&str, f64>) -> f64{
    
    let mut actual_frequency = vec![0;26];
    let chi_square_result = 0;
    for l in text.chars() {
        actual_frequency[l as usize - 97] += 1;
    } 
    let mut sum = 0.0;
    // For each letter in actual_frequency
    for (i,l) in actual_frequency.iter().enumerate() {
        let letter = char::from_u32((i+97) as u32).unwrap().to_string();
        //calculate expected value of that letter
        let expected = letter_freq[letter.as_str()] * text.len() as f64;
        // Calculate error
        let error = *l as f64 - expected;
        // Error^2
        let square = f64::powf(error, 2.0);
        sum += square/expected;
    }
    return sum;
}

fn solve(cosets: &Vec<String>, size:&usize, letter_freq: HashMap<&str,f64>) -> String {
    let mut possible_password = String::new();
    for coset in cosets {
        let mut chi_square_scores = vec![];
        // apply shifts in the coset for each letter in the alphabet
        for i in 0..26{
            let mut shifted_string = String::new();
            for letter in coset.chars() {
                let new_letter = (((letter as i32 - 97) - i) + alphabet.len() as i32) % alphabet.len() as i32; 
                shifted_string.push(alphabet.chars().nth(new_letter.try_into().unwrap()).unwrap());
            }
            let score = chi_square(&shifted_string, letter_freq.clone());
            chi_square_scores.push(score);
        }
        let min = chi_square_scores.iter().enumerate().min_by(|(_, val_a),(_, val_b)| val_a.partial_cmp(val_b).unwrap()).unwrap();
        possible_password.push(char::from_u32(min.0 as u32 +97).unwrap());
    }
    return possible_password;
}

fn challenge(_letter_freq: HashMap<&str,f64>, text: &str){
    let rgx = Regex::new(r"[^a-z]").unwrap();
    let text_formated = rgx.replace_all(text, "");

    // Make groups of mod X
    let  groups = make_groups(&text_formated,2,20);

    let dp = calculate_sd(&groups);

    let avg = groups.iter().fold(0.0, |acc, (_,_,i)| acc+i)/groups.len() as f64;
    let possible_keys: Vec<_> = groups.iter().filter(|(_, _,ic)| ic>=&(dp+avg)).map(|(key,size,_)| (key,size)).collect();
    for (cosets,possible_size) in possible_keys {
        let password = solve(cosets, possible_size,_letter_freq);
        println!("{:?}", password);
        break;
    } 
}

fn main() {
    let mut args = env::args();
    let path = args.nth(1).unwrap();
    let mut _letter_freq = fs::read_to_string(path).unwrap();
    let letter_freq:HashMap<&str, f64> = _letter_freq
    .split('\n')
    .map(|line| {
        let mut tokens = line.split(' ');
        let (letter, value) = (tokens.next().unwrap(), tokens.next().unwrap());
        (letter, value.parse::<f64>().unwrap())
    }).collect();
    let path_challenge = args.nth(0).unwrap();
    let _challenge = fs::read_to_string(path_challenge).unwrap();
    
    let _result = challenge(letter_freq, _challenge.as_str());
    // let ciphred_text = cipher("testandoessetrabalhoincrivel", "teste");
    // let deciphred_text = decipher(desafio, "arara");
    // println!("{}", deciphred_text);
}