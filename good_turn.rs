/*
    Good Turn
    -> CodeChef {rating: 238}

    Chef and Chefina are playing with dice. In one turn, both of them roll their dice at once.
    They consider a turn to be good if the sum of the numbers on their dice is greater than 66.
    Given that in a particular turn Chef and Chefina got XX and YY on their respective dice, 
    find whether the turn was good.

    https://www.codechef.com/problems/GDTURN
*/

use std::io;

fn main() {
    let mut t: String = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Unable to read T!");

    let t = match t.trim().parse::<u16>() {
        Ok(n) => n,
        _ => {
            println!("Not a valid number!");
            return ;
        }
    };

    for _ in 1..=t {
        let mut inputline: String = String::new();
        io::stdin()
            .read_line(&mut inputline)
            .expect("Unable to read input!");

        let mut words = inputline.split_whitespace();
        let x: u8 = words.next().expect("Error reading input!")
                            .parse().expect("Not a valid dice roll!");
        let y: u8 = words.next().expect("Error reading input!")
                            .parse().expect("Not a valid dice roll!");

        if x + y > 6 {
            println!("YES");
        }
        else {
            println!("NO");
        }
    }
}