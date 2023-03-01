use std::collections::HashMap;
use itertools::Itertools;
use super::helpers::read_lines;
use regex::Regex;

#[derive(Debug)]
struct Move {
    pub num: usize,
    pub from: usize,
    pub to: usize,
}

impl Move {
    pub fn from_str(line: &str) -> Option<Move> {
        let num_rgx = Regex::new(r#"[0-9]+"#).unwrap();
        if let [num, from, to] = num_rgx.find_iter(line).filter_map(|digits| digits.as_str().parse().ok()).collect_vec()[..3] {
            return Some(Move {
                num,
                from,
                to
            })
        }

        None
    }
}

fn init_crates<'a>(stacks: &'a mut HashMap<usize, Vec<char>>, input: &str) -> &'a mut HashMap<usize, Vec<char>> {
    let input = format!("{}", input);
    let res = input.chars()
        .tuples()
        .map(|(_, x, _, _)| x)
        .enumerate()
        .filter(|(_, x)| *x != ' ' && x.is_alphabetic())
        .fold(stacks, |acc, (i, letter)| {
            let existing = acc.entry(i + 1).or_insert(vec![]);
            existing.insert(0, letter);

            acc
        });


    res
}


fn move_crates<'a>(stacks: &'a mut HashMap<usize, Vec<char>>, input: &str) -> &'a mut HashMap<usize, Vec<char>> {
        let a_move = Move::from_str(input).unwrap();
    //         println!("{:?}", a_move);
    // println!("{:?}", stacks);
        if let Some(mut crates_moved) = stacks.get_mut(&a_move.from).map(|x| x.split_off(x.len() - a_move.num)) {
            // crates_moved.reverse();
            stacks.get_mut(&a_move.to).map(|to| to.append(&mut crates_moved));
        }

        stacks
}

pub fn main() {
    let Ok(lines) = read_lines("src/days/day5_input.txt") else { return };
    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    lines.fold(&mut stacks, |mut acc, line_result| {
        let Ok(line) = line_result else { return acc };

        acc = match line.chars().nth(0) {
            Some('m') => move_crates(acc, line.as_str()),
            _ => init_crates(acc, line.as_str())
        };
        acc
    });

    for stack in 1..stacks.keys().len() + 1 {
        if let Some(s) = stacks.get(&stack).and_then(|x| x.last()) {
            print!("{:?}", s);
        } else {
            print!(" ");
        }
    }

    println!("{:?}", stacks);
}
