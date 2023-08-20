/*
    Number Mirror
    -> CodeChef {rating: 200}

    Write a program that takes a number NN as the input, and prints it to the output.
*/

use std::io;

fn main() {
    let mut input_line: String = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Unable to read input!");

    let input_line = input_line.trim();

    let _N: i32 = match input_line.parse::<i32>() {
        Ok(n) => n,
        _ => {
            println!("Error: Not a number!");
            return ;
        }
    };

    println!("{}", _N);
}