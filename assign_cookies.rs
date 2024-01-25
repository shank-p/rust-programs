/*
    455. Assign Cookies
    -> Leetcode {easy}

    Assume you are an awesome parent and want to give your children some cookies. But, you should 
    give each child at most one cookie.

    Each child i has a greed factor g[i], which is the minimum size of a cookie that the child will be content 
    with; and each cookie j has a size s[j]. If s[j] >= g[i], we can assign the cookie j to the child i, and the 
    child i will be content. Your goal is to maximize the number of your content children and output the maximum 
    number.

    https://leetcode.com/problems/assign-cookies/description/?envType=daily-question&envId=2024-01-01
*/

use std::io;

pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    
    let mut ordered_cookies = s.clone();
    ordered_cookies.sort();
    let mut content_children_count = 0;

    for greed in g {
        for cookie in 0..ordered_cookies.len() {
            if ordered_cookies[cookie] >= greed {
                content_children_count += 1;
                ordered_cookies.remove(cookie);
                break;
            }
        }
    }
    
    content_children_count
}


fn main() {
    let mut g = String::new();
    let mut s = String::new();

    io::stdin()
        .read_line(&mut g)
        .expect("Unable to read input `g`");
    let g: Vec<i32> = g.split_whitespace()
                        .filter(|g| !g.is_empty())
                        .map(|g| g.parse().unwrap())
                        .collect();

    io::stdin()
        .read_line(&mut s)
        .expect("Unable to read input `g`");
    let s: Vec<i32> = s.split_whitespace()
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse().unwrap())
                        .collect();
    
    let res = find_content_children(g, s);
    println!("res : {res}");
}