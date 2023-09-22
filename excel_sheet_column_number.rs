/*
    171. Excel Sheet Column Number
    -> LeetCode {easy}

    Given a string columnTitle that represents the column title as appears in an Excel sheet, return its 
    corresponding column number.
    ex. A -> 1

    https://leetcode.com/problems/excel-sheet-column-number/
*/

use std::io;

fn title_to_number(column_title: String) -> i32 {
    if column_title.is_empty() {return 0}
    let column_title: Vec<char> = column_title.chars().collect();
    let col_title_len = column_title.len() - 1; 
    let mut col_num: i32 = 0;
    for i in 0..column_title.len() {
        println!("len : {}", col_title_len);
        println!("i : {}", i);
        println!("char : {}", (column_title[i] as i32) - 64);
        println!("pow : {}", i32::pow(26, (col_title_len - i) as u32));
        col_num += ((column_title[i] as i32) - 64) * i32::pow(26, (col_title_len - i) as u32);
        println!("col_num : {}", col_num);
    }

    col_num
}

fn main() {
    let mut column_title: String = String::new();
    io::stdin()
        .read_line(&mut column_title)
        .expect("Unable to read input 'column_title'!");

    println!("res : {}", title_to_number(String::from(column_title.trim())));
}