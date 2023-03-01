extern crate array_tool;

use std::{fs::File, io};

use array_tool::vec::*;

use super::helpers::{read_lines, remove_whitespace};

fn get_rucksack_content(line: &str) -> (Vec<char>, Vec<char>) {
    let rucksack_content = remove_whitespace(line);
    let ruck_sack_length = rucksack_content.len();
    let split_at_idx = ruck_sack_length / 2;

    let first_sack = &rucksack_content[0..split_at_idx];
    let second_sack = &rucksack_content[split_at_idx..ruck_sack_length];

    let first: Vec<char> = first_sack.chars().collect();
    let second: Vec<char> = second_sack.chars().collect();

    return (first, second);
}

fn get_priorities(lines: io::Lines<io::BufReader<File>>) -> u32 {
    let mut acc_priorities: u32 = 0;

    for line in lines {
        let (first, second) = get_rucksack_content(line.unwrap().as_str());
        let priorities = first.intersect(second).iter().fold(0 as u32, |acc, &c| {
            let score = calculate_score(c);
            acc + score
        });

        acc_priorities += priorities;
    }

    return acc_priorities;
}

fn calculate_score(item: char) -> u32 {
    return match item {
        'a'..='z' => 26 - ('z' as u32 - item as u32),
        'A'..='Z' => 52 - ('Z' as u32 - item as u32),
        _ => 0,
    };
}

fn get_badge_of_group(group_content: [String; 3]) -> char {
    let mut alphabet: Vec<char> = ('a'..'{').chain('A'..'[').collect();
    for content in group_content {
        alphabet = alphabet
            .into_iter()
            .filter(|x| content.contains(*x))
            .collect::<Vec<char>>();
    }

    return *alphabet.first().unwrap();
}

fn get_acc_badge_priority(lines: io::Lines<io::BufReader<File>>) -> u32 {
    let mut result = 0;
    for group in lines.array_chunks::<3>() {
        let badge = get_badge_of_group(group.map(|x| x.unwrap()));
        result += calculate_score(badge);
    }

    return result;
}

pub fn main() {
    if let Ok(lines) = read_lines("src/days/day3_input.txt") {
        let acc_badge = get_acc_badge_priority(lines);
        println!("{}", acc_badge);
    }

    if let Ok(lines) = read_lines("src/days/day3_input.txt") {
        let acc_priorities = get_priorities(lines);
        println!("{}", acc_priorities);
    }
}
