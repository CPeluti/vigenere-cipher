fn encoder(text: &str, key: &str) -> String {
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
fn main() {
    let ciphred_text = encoder("attackatdawn", "teste");
    println!("{}", ciphred_text);
}
