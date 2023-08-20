/*
    Game between friends [FRGAME]
    -> CodeChef {rating: 991}

    Nitin and Sobhagya were playing a game with coins. If Sobhagya has more coins then he is winning, 
    otherwise Nitin is winning. Note that this means if both Nitin and Sobhagya have the same number 
    of coins, then Nitin is winning.
    Initially Nitin has A coins while Sobhagya has B coins. Then Ritik came and gave his C coins to the 
    player who is not winning currently, after which Satyarth came and repeated the same process 
    (gave his D coins to the player who is not winning currently).
    Find the final winner of the game.

    https://www.codechef.com/problems/FRGAME
*/

use std::io;

fn add_points(mut a: i32, mut b: i32, points: i32) -> (i32, i32) {
    if a >= b {
        b += points;
    } else {
        a += points;
    }
    
    (a, b)
}

fn main() {
    let mut t: String = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Unable to read input!");

    let t: u32 = match t.trim().parse::<u32>() {
        Ok(n) => n,
        _ => {
            println!("Error: not a valid input!");
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
                                    .map(|s| s.to_string().parse::<i32>().unwrap());

        let a: i32 = nums_iter.next().unwrap_or(0);
        let b: i32 = nums_iter.next().unwrap_or(0);
        let c: i32 = nums_iter.next().unwrap_or(0);
        let d: i32 = nums_iter.next().unwrap_or(0);

        let (a, b) = add_points(a, b, c);
        let (a, b) = add_points(a, b, d);

        if a >= b {
            println!("N");
        } else {
            println!("S");
        }
    }
}