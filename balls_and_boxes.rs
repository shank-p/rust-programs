/*
    Balls and Boxes [BALLBOX]
    -> CodeChef {rating: 994}
    
    You have N balls and K boxes. You want to divide the N balls into K boxes such that:
    Each box contains ≥1 balls. No two boxes contain the same number of balls.
    Determine if it is possible to do so.

    https://www.codechef.com/problems/BALLBOX
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
            .expect("Unable to read input");

        let mut nums_iterator = input_line.trim()
                                        .split_whitespace()
                                        .map(|s| s.parse::<u32>().unwrap());
        
        let n: u32 = nums_iterator.next().unwrap_or(0);
        let k: u32 = nums_iterator.next().unwrap_or(0);

        let mut min_balls_req: u32 = (k*(k+1))/2;
        
        if n >= min_balls_req {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}