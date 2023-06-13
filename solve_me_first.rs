/*
    1. Solve Me First
    -> Hackerrank {easy}

    Compute the sum of two integers.
 */

use std::io;

fn solve_me_first(a: i32, b: i32) -> () {
    println!("{}", a + b);
}

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut a).ok().expect("Error reading input!");
    io::stdin().read_line(&mut b).ok().expect("Error reading input!");

    let a: i32 = a.trim().parse().ok().expect("Not a number!");
    let b: i32 = b.trim().parse().ok().expect("Not a number!");

    solve_me_first(a, b);
}