extern crate array_tool;

use std::{fs::File, io};
use array_tool::vec::*;
                        

fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = File::open(file_path)?;
    Ok(io::BufRead::lines(io::BufReader::new(file)))
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn main() {
    if let Ok(lines) = read_lines("input.txt")  {
        let mut acc_priorities: u32 = 0;
        for line in lines {
            let rucksack_content = remove_whitespace(line.unwrap().as_str());
            let ruck_sack_length = rucksack_content.len();
            let split_at_idx = ruck_sack_length/2;

            let first_sack = &rucksack_content[0..split_at_idx];
            let second_sack = &rucksack_content[split_at_idx..ruck_sack_length];

            let first: Vec<char> = first_sack.chars().collect();
            let second: Vec<char> = second_sack.chars().collect();

            let priorities = first.intersect(second).iter().fold(0 as u32, |acc, &c| {
                        let score = match c {
                            'a'..='z' =>  26 - ('z' as u32 - c as u32),
                            'A'..='Z' => 52 - ('Z' as u32 - c as u32),
                            _ => 0 
                        } ;
                        acc + score
            });

            acc_priorities += priorities;
        }

        println!("{}", acc_priorities);
    }
}
