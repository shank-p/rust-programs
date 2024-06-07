/*
    648. Replace Words
    -> leetcode (medium)
    https://leetcode.com/problems/replace-words/description/?envType=daily-question&envId=2024-06-07
*/


use std::io;

pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {

    let mut dictionary = dictionary.clone();
    let mut sentence_split: Vec<String> = sentence.trim()
                                                .split(' ')
                                                .map(|s| s.to_string())
                                                .collect();

    for _ in 0..dictionary.len() {
        for i in 0..dictionary.len()-1 {
            if dictionary[i].len() > dictionary[i+1].len() {
                let temp = dictionary[i].clone();
                dictionary[i] = dictionary[i+1].clone();
                dictionary[i+1] = temp;
            }
        }
    }

    for i in 0..dictionary.len() {
        for idx in 0..sentence_split.len() {
            if sentence_split[idx].starts_with(&dictionary[i]) {
                sentence_split[idx] = dictionary[i].clone();
            }
        }
    }
    sentence_split.join(" ")
}

pub fn main() {
    let mut dictionary = String::new();
    io::stdin()
        .read_line(&mut dictionary)
        .expect("Unable to read input for `dictionary`!");
    let dictionary: Vec<String> = dictionary.trim()
                                            .split(' ')
                                            .filter(|s| !s.is_empty())
                                            .map(|s| s.to_string())
                                            .collect();
    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Unable to read input for `sentence`!");

    let res: String = replace_words(dictionary, sentence);
    println!("-->> res : {res}");
}