fn cipher(text: &str, key: &str) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
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

fn decipher(text: &str, key: &str) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut deciphred_text = String::from("");
    for (i, letter) in text.chars().enumerate() {
        let let_value = letter as i32-97;
        let key_let = key.as_bytes()[i%key.len()] as i32-97;
        let new_index = (let_value-key_let + (alphabet.len()) as i32) % (alphabet.len()) as i32;
        
        let deciphred_letter = alphabet.chars().nth(new_index.try_into().unwrap()).unwrap();
        deciphred_text.push(deciphred_letter);
    }
    return deciphred_text;
}

fn main() {
    let ciphred_text = cipher("testandoessetrabalhoincrivel", "teste");
    let deciphred_text = decipher(&ciphred_text, "teste");
    println!("{}", deciphred_text);
}
