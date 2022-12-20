use clap::Parser;
use std::{path::Path, io, io::{ Lines, BufReader, BufRead}, fs::File, cmp};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg()]
    part: usize,

    /// Number of times to greet
    #[arg(default_value = "input")]
    in_file: String,
}

fn main() {
    let args = Args::parse();
    match args.part {
        1 => part1(args.in_file),
        2 => part2(args.in_file),
        _ => println!("There is no part {}", args.part),
    }
}
fn part1(in_path_str: String) {
    println!("{}", in_path_str);
    let lines = readlines(in_path_str).expect("Failed opening input file");
    let mut cur_cals = 0usize;
    let mut total = 0usize;
    for line_result in lines {
        let line = line_result.expect("Failed to read line");
        if line.trim().is_empty() {
            total = cmp::max(total, cur_cals);
            cur_cals = 0;
        } else {
            cur_cals += line.parse::<usize>().expect("line is not a positive integer");
        };
    }
    println!("{}", total)
}
fn part2(in_path_str: String) {
    let in_path = Path::new(&in_path_str);
    let lines = readlines(in_path).expect("Failed opening input file");
    let mut cur_cals = 0usize;
    let mut top_cals = 0usize;
    let mut second_cals = 0usize;
    let mut third_cals = 0usize;
    for line_result in lines {
        let line = line_result.expect("Failed to read line");
        if line.trim().is_empty() {
            if cur_cals > third_cals {
                third_cals = cur_cals
            }
            if third_cals > second_cals {
                let temp = second_cals;
                second_cals = third_cals;
                third_cals = temp;
            }
            if second_cals > top_cals {
                let temp = top_cals;
                top_cals = second_cals;
                second_cals = temp;
            }
            cur_cals = 0;
        } else {
            cur_cals += line.parse::<usize>().expect("line is not a positive integer");
        };
    }
    println!("First {}", top_cals);
    println!("Second {}", second_cals);
    println!("Third {}", third_cals);
    println!("Total {}", top_cals+second_cals + third_cals)
}
fn readlines<P>(filename: P) -> io::Result<Lines<BufReader<File>>> 
    where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
