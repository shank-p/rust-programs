/*
    Bomb the base [BOMBTHEBASE]
    -> CodeChef {rating 982}

    In Chefland, there are N houses numbered from 1 to N, ithith house has a defence system having 
    strength Ai​.
    Chef suspects a bomb drop on one of the houses very soon. A bomb with attack strength X can destroy
    the ith house, if the defence system of the ith house Ai​, is strictly less than X.
    Also, when the ith house is destroyed due to the bomb, all houses with indices j such that 1≤j<i 
    get destroyed as well irrespective of their defence system.
    Given one bomb with attack strength X, find the maximum number of houses that can get destroyed.

    https://www.codechef.com/problems/BOMBTHEBASE

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
            println!("Not a valid input for 'T' !");
            return 
        }
    };

    for _ in 1..=t {
        let mut input_line: String = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Unable to read input!");

        let mut input_line = input_line.trim()
                                    .split_whitespace()
                                    .map(|s| s.to_string().parse::<u32>().unwrap());

        let n: u32 = input_line.next().unwrap_or(0);
        let x: u32 = input_line.next().unwrap_or(0);

        let mut input_nums: String = String::new();
        io::stdin()
            .read_line(&mut input_nums)
            .expect("Unabele to read input!");

        let h: Vec<u32> = input_nums.trim()
                                    .split_whitespace()
                                    .filter(|s| !s.is_empty())
                                    .map(|s| s.to_string().parse::<u32>().unwrap())
                                    .collect();
        
        for i in (0..h.len()).rev() {
            if x > h[i] {
                println!("{}", i+1);
                break
            } 
            if i == 0 {
                println!("0");
            }
        }
    }
}