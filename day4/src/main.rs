use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    part: usize,

    #[arg(default_value = "input")]
    in_path_str: String,
}

fn main() {
    let args = Args::parse();
    match args.part {
        1 => part1(args.in_path_str),
        2 => part2(args.in_path_str),
        _ => println!("There is no part {}!", args.part),
    }
}

struct Range {
    max: usize,
    min: usize,
}
fn part1(in_path_str: String) {
    let lines = read_lines(&in_path_str)
        .expect(format!("Could not open file \"{}\"", in_path_str).as_str());
    let mut pairs_contain = 0;
    for line_result in lines {
        let line = line_result.expect("Couldn't read line");
        let elf_range_strs = line.split(',');
        let mut elf_ranges = elf_range_strs.map(|range_str| {
            let mut min_max = range_str.split('-');
            Range {
                min: min_max.next().unwrap().parse().unwrap(),
                max: min_max.next().unwrap().parse().unwrap(),
            }
        });
        let first_range = elf_ranges.next().unwrap();
        let second_range = elf_ranges.next().unwrap();
        if first_range.contains_other_range(&second_range) {
            pairs_contain += 1;
        } else if second_range.contains_other_range(&first_range) {
            pairs_contain += 1;
        }
    }
    println!("{}", pairs_contain)
}
fn part2(in_path_str: String) {
    let lines = read_lines(&in_path_str)
        .expect(format!("Could not open file \"{}\"", in_path_str).as_str());
    let mut pairs_overlap = 0;
    for line_result in lines {
        let line = line_result.expect("Couldn't read line");
        let elf_range_strs = line.split(',');
        let mut elf_ranges = elf_range_strs.map(|range_str| {
            let mut min_max = range_str.split('-');
            Range {
                min: min_max.next().unwrap().parse().unwrap(),
                max: min_max.next().unwrap().parse().unwrap(),
            }
        });
        let first_range = elf_ranges.next().unwrap();
        let second_range = elf_ranges.next().unwrap();
        if first_range.overlaps_other_range(&second_range) || second_range.overlaps_other_range(&first_range){
            pairs_overlap += 1;
        }
        
    }
    println!("{}", pairs_overlap)
}
impl Range {
    fn contains_other_range(&self, other: &Range) -> bool {
        self.min <= other.min && self.max >= other.max
    }
    fn overlaps_other_range(&self, other: &Range) -> bool {
        (self.min <= other.min && self.max >= other.min) || (self.min <= other.max && self.max >= other.max)
    }
}
fn read_lines<P>(in_file_path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(in_file_path)?;
    Ok(BufReader::new(file).lines())
}
