/*
    Kitchen Timetable [KTTABLE]
    -> CodeChef {rating: 997}

    There are N students living in the dormitory of Berland State University. Each of them sometimes 
    wants to use the kitchen, so the head of the dormitory came up with a timetable for kitchen's usage 
    in order to avoid the conflicts:
    ->  The first student starts to use the kitchen at the time 0 and should finish the cooking not later 
        than at the time A1.
    ->  The second student starts to use the kitchen at the time A1 and should finish the cooking not later 
        than at the time A2. And so on.
    ->  The N-th student starts to use the kitchen at the time AN-1 and should finish the cooking not later 
        than at the time AN
    The holidays in Berland are approaching, so today each of these N students wants to cook some 
    pancakes. The i-th student needs Bi units of time to cook.
    The students have understood that probably not all of them will be able to cook everything they want. 
    How many students will be able to cook without violating the schedule?

    https://www.codechef.com/problems/KTTABLE
*/

use std::io;

fn main() {
    let mut t: String = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Unable to read input!");

    let t: u8 = match t.trim().parse::<u8>() {
        Ok(n) => n,
        _ => {
            println!("Not a valid input for 'T'!");
            return 
        }
    };

    for _ in 1..=t {
        let mut input_line: String = String::new();
    
        io::stdin()
            .read_line(&mut input_line)
            .expect("Unable to read input!");
        let _n: u32 = match input_line.trim().parse::<u32>() {
            Ok(n) => n,
            _ => {
                println!("Not a valid input for 'N'!");
                return 
            }
        };

        input_line.clear();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Unable to read input!");
        let a_nums: Vec<u32> = input_line.trim()
                                        .split_whitespace()
                                        .map(|s| s.to_string().parse::<u32>().unwrap())
                                        .collect();

        input_line.clear();
        io::stdin()
        .read_line(&mut input_line)
        .expect("Unable to read input!");
        let b_nums: Vec<u32> = input_line.trim()
                                        .split_whitespace()
                                        .map(|s| s.parse::<u32>().unwrap())
                                        .collect();

        let mut student_can_cook: u32 = 0;

        if a_nums[0]-0 >= b_nums[0] {
            student_can_cook += 1;
        }
        for i in 0..a_nums.len()-1 {
            if a_nums[i+1]-a_nums[i] >= b_nums[i+1] {
                student_can_cook += 1;
            }
        }
        println!("Students who can cook pancakes: {}", student_can_cook);
    }
}