/*
    2810. Faulty Keyboard
    -> LeetCode {easy}

    Your laptop keyboard is faulty, and whenever you type a character 'i' on it, it reverses the string that you 
    have written. Typing other characters works as expected.
    You are given a 0-indexed string s, and you type each character of s using your faulty keyboard.
    Return the final string that will be present on your laptop screen.

    https://leetcode.com/problems/faulty-keyboard/description/
*/

use std::io;

fn main() {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Unable to read input `s`!");
    let s = s.trim();
    let mut v = Vec::new();
    for i in s.chars() {
        match i {
            'i' => v.reverse(),
            _ => v.push(i)
        };
    }
    let s: String = v.iter().collect();
    println!("res : {s}");
}
