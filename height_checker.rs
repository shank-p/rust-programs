/*
    1051. Height Checker
    -> leetcode (easy)
    https://leetcode.com/problems/height-checker/description/?envType=daily-question&envId=2024-06-10
*/

use std::io;

pub fn height_checker(heights: Vec<i32>) -> i32 {
    let mut anamoly = 0;
    let mut sorted_heights = heights.clone();
    sorted_heights.sort();
    for i in 0..heights.len() {
        if sorted_heights[i] != heights[i] {
            anamoly += 1;
        }
    }
    anamoly
}

fn main() {
    let mut heights = String::new();
    io::stdin()
        .read_line(&mut heights)
        .expect("Unable to read input `heights`!");
    let heights = heights.split_whitespace()
                        .map(|s| s.to_string().parse::<i32>().unwrap())
                        .collect();

    let res = height_checker(heights);
    println!("-->> res : {res}");
}