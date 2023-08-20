/*
    Add Two Numbers
    -> CodeChef {rating: 242}

    Your task is very simple: given two integers AA and BB, 
    write a program to add these two numbers and output the sum.

    https://www.codechef.com/problems/FLOW001
*/

use std::io;

fn main() {
    let mut t: String = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Unable to read input!");

    let t: u32 = match t.trim().parse::<u32>() {
        Ok(n) => n,
        _ => {
            println!("Not a valid input!");
            return ;
        }
    };

    for _ in 1..=t {
        let mut input_line: String = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Unable to read input!");

        let mut nums: Vec<i32> = input_line.trim()
                                        .split(" ")
                                        .filter(|s| !s.is_empty())
                                        .map(|s| s.to_string().parse::<i32>().unwrap())
                                        .collect();                            
        
        if nums.len() < 2 {
            for _ in 0..2-nums.len() {
                nums.push(0);
            }
        }

        println!("{}", nums[0]+nums[1]);
    };
}