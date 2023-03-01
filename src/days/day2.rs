use crate::days::helpers::read_lines;

use super::helpers::remove_whitespace;

fn get_choices(line: &str) -> (RPS, RPS) {
    return get_rps(line).unwrap();
}

fn get_rps(choice: &str) -> Result<(RPS, RPS), ()> {
    if let [opp_choice_str, result] = choice.split(" ").collect::<Vec<&str>>()[0..2] {
        let opp_choice = match opp_choice_str {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!("Not supported!"),
        };

        return Ok((
            opp_choice,
            match remove_whitespace(result).as_str() {
                "X" => get_loser(opp_choice),
                "Y" => opp_choice,
                "Z" => get_winner(opp_choice),
                _ => panic!("Not supported"),
            },
        ));
    }
    return Err(());
}

fn get_winner(choice: RPS) -> RPS {
    return match choice {
        RPS::Rock => RPS::Paper,
        RPS::Paper => RPS::Scissors,
        RPS::Scissors => RPS::Rock,
    };
}

fn get_loser(choice: RPS) -> RPS {
    return match choice {
        RPS::Rock => RPS::Scissors,
        RPS::Paper => RPS::Rock,
        RPS::Scissors => RPS::Paper,
    };
}

fn calculate_score(opp: RPS, me: RPS) -> i32 {
    if opp == me {
        return 3 + me as i32;
    }
    return match (opp, me) {
        (RPS::Scissors, RPS::Rock) => 6,
        (RPS::Rock, RPS::Paper) => 6,
        (RPS::Paper, RPS::Scissors) => 6,
        (_, __) => 0,
    } + me as i32;
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

pub fn main() {
    let Ok(lines) = read_lines("src/days/day2_input.txt") else { return };
    let mut score = 0;
    for line in lines {
        if let Ok(l) = line {
            let (opp, me) = get_choices(l.as_str());
            score += calculate_score(opp, me);
        }
    }
    println!("{}", score);
}
