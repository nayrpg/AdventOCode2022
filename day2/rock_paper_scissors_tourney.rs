use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input2") {
        // Consumes the iterator, returns an (Optional) String
        let mut total: usize = 0;
        // let mut plays: Vec<Play> = Vec::new();
        for line in lines {
            if let Ok(ip) = line {
                let play_str: Vec<&str> = ip.split(' ').collect();
                let play = Play::new(play_str[0], play_str[1]);
                total += match play.result {
                    MatchResult::Win(score) => score,
                    MatchResult::Lose(score) => score,
                    MatchResult::Draw(score) => score,
                    _ => panic!("Score Invalid"),
                };
                // plays.push(play);
                
            }
        }
        println!("{}",total);
        
    }
}
#[derive(Debug)]
struct Play {
    in_str: String,
    out_str: String,
    result: MatchResult,
}
impl Play {
    fn new(in_str: &str, out_str: &str) -> Play {
        let mut play = Play {
            in_str: String::from(in_str),
            out_str: String::from(out_str),
            result: MatchResult::NotScored,
        };
        play.score_pt2();
        play
    }
    fn score(&mut self) {
        self.result = match self.in_str.as_str() {
            "A" => match self.out_str.as_str() {
                "X" => MatchResult::Draw(1 + 3),
                "Y" => MatchResult::Win(2 + 6),
                "Z" => MatchResult::Lose(3 + 0),
                &_ => panic!("Unexpected input"),
            },
            "B" => match self.out_str.as_str() {
                "X" => MatchResult::Lose(1 + 0),
                "Y" => MatchResult::Draw(2 + 3),
                "Z" => MatchResult::Win(3 + 6),
                &_ => panic!("Unexpected input"),
            },
            "C" => match self.out_str.as_str() {
                "X" => MatchResult::Win(1 + 6),
                "Y" => MatchResult::Lose(2 + 0),
                "Z" => MatchResult::Draw(3 + 3),
                &_ => panic!("Unexpected input"),
            },
            &_ => panic!("Unexpected input"),
        };
    }
    fn score_pt2(&mut self) {
        self.result = match self.in_str.as_str() {
            "A" => match self.out_str.as_str() {
                "X" => MatchResult::Lose(3 + 0),
                "Y" => MatchResult::Draw(1 + 3),
                "Z" => MatchResult::Win(2 + 6),
                &_ => panic!("Unexpected input"),
            },
            "B" => match self.out_str.as_str() {
                "X" => MatchResult::Lose(1 + 0),
                "Y" => MatchResult::Draw(2 + 3),
                "Z" => MatchResult::Win(3 + 6),
                &_ => panic!("Unexpected input"),
            },
            "C" => match self.out_str.as_str() {
                "X" => MatchResult::Lose(2 + 0),
                "Y" => MatchResult::Draw(3 + 3),
                "Z" => MatchResult::Win(1 + 6),
                &_ => panic!("Unexpected input"),
            },
            &_ => panic!("Unexpected input"),
        };
    }
}

#[derive(Debug)]
enum MatchResult {
    NotScored,
    Lose(usize),
    Draw(usize),
    Win(usize),
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
