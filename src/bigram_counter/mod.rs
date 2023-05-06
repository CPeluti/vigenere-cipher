use std::{collections::HashMap,fs, io::{self, Write}};
use indicatif::ProgressBar;
pub fn count_bigrams(path: &String, path_output: &String) -> Result<HashMap<Vec<u8>, u32>, io::Error>{

    let file = fs::read_to_string(path)?;
    
    // convert and remove letters
    let processed_text: Vec<u8> = file.chars().flat_map(|letter|{
        match letter {
            'ú'|'û'|'ù'|'ü' => Some('u'),
            'í'|'î'|'ì'|'ï' => Some('i'),
            'ó'|'ò'|'ô'|'õ'|'ố'=> Some('o'),
            'é'|'ê'|'è' => Some('e'),
            'á'|'ã'|'â'|'à'|'ẫ'|'å' => Some('a'),
            'ç'=> Some('c'),
            'a'..='z' => Some(letter),
            'A'..='Z' => Some((letter as u8 + 32) as char),
            _=> None
        }
    }).map(|l| l as u8).collect();

    // count each bigram in the text
    

    let mut nums = vec![];
    for l1 in b'a'..=b'z'{
        for l2 in b'a'..=b'z'{
            nums.push(vec![l1,l2]);
        }
    }

    let pb = ProgressBar::new((processed_text.len()-1) as u64);

    let mut dict: HashMap<Vec<u8>, f64> = HashMap::new();
    for i in 0..processed_text.len()-1{
        let key = vec![processed_text[i], processed_text[i+1]];
        dict.entry(key).and_modify(|value| *value+=1.0).or_insert(1.0);
        pb.inc(1);
    }
    pb.finish_with_message("done");
    nums.iter().for_each(|bigram| {
        dict.entry(bigram.clone()).or_insert(1.0);
    });


    dict=dict.into_iter()
    .map(|(key,value)| {
        let calc = value as f64/processed_text.len() as f64;
        (key, calc.ln())
    })
    .collect();

    let (_,&max_value) = dict.iter().max_by(|(_,value),(_,value1)| value.partial_cmp(value1).unwrap()).unwrap();
    let (_,&min_value) = dict.iter().min_by(|(_,value),(_,value1)| value.partial_cmp(value1).unwrap()).unwrap();
    
    let dict_normalized: HashMap<Vec<u8>, u32>= dict.into_iter()
    .map(|(key,value)| (key,(1000000 as f64*((value-min_value)/(max_value-min_value))).round() as u32))
    .collect();

    let mut file = fs::File::create(path_output)?;

    dict_normalized.iter().for_each(|(bigram, count)|{
        file.write_all(format!("{} {}\n",String::from_utf8(bigram.clone()).unwrap(), count).as_bytes()).unwrap();
    });

    
    Ok(dict_normalized)

}
