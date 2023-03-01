use itertools::Itertools;
use std::fs::read_to_string;

pub fn main() {
    let Ok(input) = read_to_string("src/days/day8_input.txt") else {return};

    let mat = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let mut num_visible = 0;
    for row in 0..mat.len() {
        for col in 0..mat[row].len() {
            if is_visible(&mat, row, col) {
                num_visible += 1;
            }
        }
    }

    println!("{}", num_visible);

    let mut curr_high_score = 0;
    for row in 0..mat.len() {
        for col in 0..mat[row].len() {
            let score = calculate_scenic_score_all_direction(&mat, row, col);
            if score > curr_high_score {
                curr_high_score = score;
            }
        }
    }

    println!("{}", curr_high_score);
}

fn is_visible(mat: &Vec<Vec<u32>>, row: usize, col: usize) -> bool {
    if row == 0 || col == 0 || row == mat.len() - 1 || col == mat[0].len() - 1 {
        return true;
    }

    let curr = mat[row][col];

    let left_visible = mat[row][..col].iter().max().unwrap() < &curr;
    let right_visible = mat[row][col + 1..].iter().max().unwrap() < &curr;
    let up_visible = mat[..row].iter().map(|r| r[col]).max().unwrap() < curr;
    let down_visible = mat[row + 1..].iter().map(|r| r[col]).max().unwrap() < curr;

    return left_visible || right_visible || up_visible || down_visible;
}

fn calculate_scenic_score_all_direction(mat: &Vec<Vec<u32>>, row: usize, col: usize) -> u64 {
    if row == 0 || col == 0 || row == mat.len() - 1 || col == mat[0].len() - 1 {
        return 0;
    }

    let len = mat[row].len();
    let curr = mat[row][col];

    let left: u64 = 1 + mat[row][1..col]
        .iter()
        .rev()
        .map_while(|&v| if v < curr { Some(v) } else { None })
        .count() as u64;
    let right: u64 = 1 + mat[row][col + 1..len - 1]
        .iter()
        .map_while(|&v| if v < curr { Some(v) } else { None })
        .count() as u64;
    let up: u64 = 1 + mat[1..row]
        .iter()
        .rev()
        .map(|v| v[col])
        .map_while(|v| if v < curr { Some(v) } else { None })
        .count() as u64;
    let down: u64 = 1 + mat[row + 1..len - 1]
        .iter()
        .map(|v| v[col])
        .map_while(|v| if v < curr { Some(v) } else { None })
        .count() as u64;
    left * right * up * down
}

