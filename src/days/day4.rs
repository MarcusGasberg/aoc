use std::ops::Range;

use crate::days::helpers::{read_lines, remove_whitespace};

#[derive(Debug)]
struct Elve {
    lower_bound: u32,
    upper_bound: u32,
}

impl Elve {
    fn new(bounds: &str) -> Elve {
        if let [lower_bound, upper_bound] = bounds
            .split('-')
            .map(|x| u32::from_str_radix(x, 10).unwrap())
            .collect::<Vec<u32>>()[0..2]
        {
            return Elve {
                lower_bound,
                upper_bound: upper_bound + 1,
            };
        } else {
            todo!()
        }
    }

    fn get_range(&self) -> Range<u32> {
        return self.lower_bound..self.upper_bound;
    }

    pub fn overlaps(&self, other: &Elve) -> bool {
        let mut range = self.get_range();
        let other_range = other.get_range();
        return range.any(|x| other_range.contains(&x));
    }
}

pub fn main() {
    if let Ok(lines) = read_lines("src/days/day4_input.txt") {
        let mut num_overlapping = 0;
        for line in lines {
            let elve_bounds = line
                .unwrap()
                .split(",")
                .map(|x| remove_whitespace(x))
                .collect::<Vec<String>>();
            let elves = &elve_bounds
                .iter()
                .map(|x| Elve::new(x))
                .collect::<Vec<Elve>>()[0..2];

            num_overlapping += if elves[0].overlaps(&elves[1]) || elves[1].overlaps(&elves[0]) {
                1
            } else {
                0
            };
        }

        println!("{}", num_overlapping);
    }
}
