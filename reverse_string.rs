/*
    344. Reverse String
    -> leetcode (easy)
    https://leetcode.com/problems/reverse-string/description/?envType=daily-question&envId=2024-06-02
*/


use std::io;


pub fn reverse_string(s: &mut Vec<char>) {
    let n = s.len();
    for i in 0..n/2 {
        s.swap(i, n-i-1);
    }
}

fn main() {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Unable to read inpt `s`!");
    let mut s = s.split_whitespace()
                .map(|s| s.to_string().parse::<char>().unwrap())
                .collect();
    
    reverse_string(&mut s);
} 