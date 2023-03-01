// use std::{
//     fs::File,
//     io::{self, BufRead},
//     path::Path,
// };
//
// pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }
//
// pub fn remove_whitespace(s: &str) -> String {
//     s.chars().filter(|c| !c.is_whitespace()).collect()
// }
