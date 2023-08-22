/*
    Squats [SQUATS]
    -> CodeChef {rating 249}

    Somu went to the gym today. He decided to do X sets of squats. Each set consists of 15 squats. 
    Determine the total number of squats that he did today.

    https://www.codechef.com/problems/SQUATS

*/

use std::io;

fn main() {

    let mut t: String = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Unable to read input!");

    let t: u32 = match t.trim().parse::<u32>() {
        Ok(t) => t,
        _ => {
            println!("Not a valid input for 'T' !");
            return 
        }
    };


    for _ in 1..=t {
        let mut input_string: String = String::new();
        io::stdin()
            .read_line(&mut input_string)
            .expect("Unable to read input!");

        let x: u32 = match input_string.trim().parse::<u32>() {
            Ok(x) => x,
            _ => {
                println!("Not a valid input for 'X' !");
                return 
            }
        };

        println!("{}", x*15);
    }

}