/*
    1122. Relative Sort Array
    -> leetcode (easy)
    https://leetcode.com/problems/relative-sort-array/description/?envType=daily-question&envId=2024-06-11
*/

use std::io;
use std::collections::HashMap;

pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut arr1_counter: HashMap<i32, i32> = HashMap::new();
    let mut relative_sorted_arr1: Vec<i32> = Vec::new();
    for num in arr1 {
        arr1_counter.entry(num).and_modify(|counter| *counter += 1).or_insert(1);
    }
    for ref_num in &arr2 {
        if arr1_counter.contains_key(ref_num) {
            relative_sorted_arr1.extend_from_slice(&vec![*ref_num;arr1_counter[ref_num] as usize]);
        }
        arr1_counter.remove(&ref_num);
    }
    let mut sorted_remaining_arr1_values: Vec<&i32> = arr1_counter.keys().collect();
    sorted_remaining_arr1_values.sort(); 
    for extra_num in sorted_remaining_arr1_values {
        relative_sorted_arr1.extend_from_slice(&vec![*extra_num;arr1_counter[extra_num] as usize]);
    }
    relative_sorted_arr1
}

fn main() {
    let mut arr1 = String::new();
    io::stdin()
        .read_line(&mut arr1)
        .expect("Unable to read input for `arr1`!");
    let mut arr2 = String::new();
    io::stdin()
        .read_line(&mut arr2)
        .expect("Unable to read input for `arr2`!");
    let arr1: Vec<i32> = arr1.split_whitespace()
                            .map(|s| s.to_string().parse::<i32>().unwrap())
                            .collect();
    let arr2: Vec<i32> = arr2.split_whitespace()
                            .map(|s| s.to_string().parse::<i32>().unwrap())
                            .collect();
    
    let res = relative_sort_array(arr1, arr2);
    println!("-->> res : {res:?}");
}
