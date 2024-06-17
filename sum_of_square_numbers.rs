/*
    633. Sum of Square Numbers
    -> leetcode (medium)
    https://leetcode.com/problems/sum-of-square-numbers/description/?envType=daily-question&envId=2024-06-17
*/

use std::io;
use std::cmp::Ordering;

pub fn judge_square_sum(c: i32) -> bool {
    let c_sqrt = (c as f64).sqrt() as i64;
    let (mut left, mut right): (i64, i64) = (0, c_sqrt);
    while left <= right {
        let square_sum = left*left + right*right;
        match square_sum.cmp(&(c as i64)) {
            Ordering::Equal => {return true;},
            Ordering::Less => {left += 1;},
            Ordering::Greater => {right -= 1;}
        }
    }
    false
}

fn main() {
    let mut c = String::new();
    io::stdin()
        .read_line(&mut c)
        .expect("Unabel to read input `c`!");
    let c = c.trim().parse::<i32>().unwrap();

    let res = judge_square_sum(c);
    println!("-->> res : {res}");
}