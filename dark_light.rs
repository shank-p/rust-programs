/*
    Dark Light [DARLIG]
    -> CodeChef {rating 994}

    Tonmoy has a special torch. The torch has 4 levels numbered 1 to 4 and 2 states (On and Off). 
    Levels 1,2 and 3 correspond to the On state while level 4 corresponds to the Off state.
    The levels of the torch can be changed as:
        -> Level 1 changes to level 2.
        -> Level 2 changes to level 3.
        -> Level 3 changes to level 4.
        -> Level 4 changes to level 1.
    Given the initial state as KK and the number of changes made in the levels as N, find the final state 
    of the torch. If the final state cannot be determined, print Ambiguous instead.

    https://www.codechef.com/problems/DARLIG
*/

use std::io;

fn main() {
    let mut t: String = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Unable to read input!");

    let t: u64 = match t.trim().parse::<u64>() {
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

        let mut nums_iter = input_line.trim()
                                .split_whitespace()
                                .map(|s| s.parse::<u64>().unwrap());

        let n: u64 = nums_iter.next().unwrap_or(0);
        let k: u64 = nums_iter.next().unwrap_or(0);

        if n%4 == 0 {
            if k==1 {
                println!("On");
            } else {
                println!("Off");
            }
        }
        
        else {
            if k==0 {
                println!("On")
            } else {
                println!("Ambiguous");
            }
        }
    }
}
