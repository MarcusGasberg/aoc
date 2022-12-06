use std::{
    fs::File,
    io::{self, BufRead }, path::Path,
};

#[derive(Debug, Clone)]
struct Elve {
    pub cal: i64,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn main() {
    let Ok(lines) = read_lines("src/input.txt") else { return };

    let mut elves = vec![];
    let mut current_elve_cals: i64 = 0; 
    for line in lines {
        match line  {
            Ok(line_content) => {
                match line_content.as_str()  {
                    "" => {
                        elves.push(Elve {
                            cal: current_elve_cals
                        });
                        current_elve_cals = 0;
                    },
                    cal => {
                        let num = &remove_whitespace(cal);
                        current_elve_cals += match i64::from_str_radix(num, 10) {
                            Ok(num) => num,
                            Err(_) => 0,
                        };
                    }
                }
            },
            Err(_err) => continue
        }
    }


    elves.sort_by(|x, y| y.cal.cmp(&x.cal));

    let top_elve_cal = elves[0].cal;
    println!("Top elve cal: {}", top_elve_cal);

    let top_three_elves_cals = elves[0..3].iter().map(|e| e.cal).sum::<i64>();
    println!("Top three elves cal: {}", top_three_elves_cals);
}
