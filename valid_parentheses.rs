/*
    20. Valid Parentheses
    -> LeetCode {easy}

    Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

    An input string is valid if:
    -> Open brackets must be closed by the same type of brackets.
    -> Open brackets must be closed in the correct order.
    -> Every close bracket has a corresponding open bracket of the same type.

*/

use std::io::{self, BufRead};
use std::collections::HashMap;

fn is_valid(s: String) -> bool {
    let parentheses: HashMap<char, char> = ([('(', ')'), ('{', '}'), ('[', ']')])
                                                .iter()
                                                .cloned()
                                                .collect();
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        match stack.last() {
            Some(token) => {
                if ch == match parentheses.get(token){
                            Some(&val) => val,
                            _ => return false,
                        } {
                    stack.pop();
                }
                else {
                    stack.push(ch);
                }
            }

            _ => {
                stack.push(ch);
            }
        }
    }

    stack.is_empty()       
}

fn main() {

    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let s = match stdin_iterator.next(){
                        Some(Ok(line)) => line,
                        _ => return,
                    };

    println!("input string : {:?}", s);
    
    let result: bool = is_valid(s);
    println!("result: {}", result);
}