/*
    1496. Path Crossing
    -> LeetCode

    Given a string path, where path[i] = 'N', 'S', 'E' or 'W', each representing moving one unit north, south,
    east, or west, respectively. You start at the origin (0, 0) on a 2D plane and walk on the path specified 
    by path.
    Return true if the path crosses itself at any point, that is, if at any time you are on a location you have 
    previously visited. Return false otherwise.

    https://leetcode.com/problems/path-crossing/description/?envType=daily-question&envId=2023-12-23
*/

use std::io;

// Time: O(n) | Space: O(n)
pub fn is_path_crossing(path: String) -> bool {
    let mut travelled_path: Vec<(i32, i32)> = Vec::new();
    let (mut x, mut y): (i32, i32) = (0, 0);
    travelled_path.push((x, y));
    for direction in path.chars() {
        match direction {
            'N' => y += 1,
            'S' => y -= 1,
            'W' => x -= 1,
            _ => x += 1,
        }
        if travelled_path.contains(&(x, y))  {
            return true; 
        } else {
            travelled_path.push((x, y));
        }
    }
    false
}


fn main() {
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Unable to read input `path`");
    let path: String = path.trim().to_string(); 
    let res = is_path_crossing(path);
    println!("res : {res}");
}