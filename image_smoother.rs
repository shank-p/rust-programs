/*
    661. Image Smoother
    -> LeetCode

    An image smoother is a filter of the size 3 x 3 that can be applied to each cell of an image by rounding down
    the average of the cell and the eight surrounding cells (i.e., the average of the nine cells in the blue 
    smoother). If one or more of the surrounding cells of a cell is not present, we do not consider it in the 
    average (i.e., the average of the four cells in the red smoother).
    Given an m x n integer matrix img representing the grayscale of an image, return the image after applying the 
    smoother on each cell of it.

    https://leetcode.com/problems/image-smoother/description/?envType=daily-question&envId=2023-12-19
*/

use std::io;

pub fn calc_avg(row_st: usize, row_en: usize, col_st: usize, col_en: usize, img: &Vec<Vec<i32>>) -> i32 {    
    let mut sum = 0;
    let mut count = 0;
    for r in row_st..=row_en {
        for c in col_st..=col_en {
            sum += img[r][c];
            count += 1;
        }
    }
    let avg = sum/count;
    avg
}

pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let m: usize = img.len();
    let n: usize = img[0].len(); 
    
    // Initailized a null vec! of same size
    let mut smooth_img: Vec<Vec<i32>> = vec![vec![0;n];m]; 

    for (row_idx, row) in img.iter().enumerate() {
        let (row_st, row_en): (usize, usize) = match m {
            1 => (0, 0),        // 1st row index in 1 row matrix
            _ => match row_idx {
                    0 => (row_idx, row_idx+1),
                    _ if row_idx == m-1 => (row_idx-1, row_idx),
                    _ => (row_idx-1, row_idx+1)
                },
        };
        for (col_idx, &pixel) in row.iter().enumerate() {  
            let (col_st, col_en): (usize, usize) = match n {
                1 => (0, 0),        // 1st col index in 1 col matrix
                _ => match col_idx {
                        0 => (col_idx, col_idx+1),
                        _ if col_idx == n-1 => (col_idx-1, col_idx),
                        _ => (col_idx-1, col_idx+1)
                    },
            };
            smooth_img[row_idx][col_idx] = calc_avg(row_st, row_en, col_st, col_en, &img);
        } 
    }

    smooth_img
}

fn main() {
    let mut img_size = String::new();
    io::stdin()
        .read_line(&mut img_size)
        .expect("Unable to read input `img-size`");
    let (m, n): (u32, u32) = {
        let mut dimensions = img_size.split_whitespace();
        (
            dimensions.next().unwrap().parse().unwrap(),
            dimensions.next().unwrap().parse().unwrap(),
        )
    };

    let mut raw_img = String::new();
    io::stdin()
        .read_line(&mut raw_img)
        .expect("Unable to read input `img`");
    let raw_img: Vec<i32> = raw_img.split_whitespace()
                                .filter(|s| !s.is_empty())
                                .map(|s| s.parse().unwrap())
                                .collect();
    
    let mut img: Vec<Vec<i32>> = Vec::new();
    let n: usize = n as usize;
    let mut start_idx: usize = 0;
    for _ in 0..m {
        img.push(raw_img[start_idx..start_idx+n].to_vec());
        start_idx += n;
    }
    
    let res = image_smoother(img);
    println!("res : {:?}", res);
}