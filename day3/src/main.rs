use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path, collections::HashSet,
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

fn part1(in_path_str: String) {
    let lines = read_lines(&in_path_str)
        .expect(format!("Could not open file \"{}\"", in_path_str).as_str());
    let mut total_prior = 0usize;
    for line_result in lines {
        let line = line_result.expect("Couldn't read line");
        let (sec1, sec2) = line.split_at(line.len() / 2);
        let bag_sec1:HashSet<char> = sec1.chars().collect::<>();
        let mut shared_priority = 0u8;
        for c in sec2.chars() {
            if bag_sec1.get(&c).is_some() {
                shared_priority = char_to_priority(&c);
            }
        }
        total_prior += shared_priority as usize
    }
    println!("Total priority: {}", total_prior)
}
fn part2(in_path_str: String) {
    let lines = read_lines(&in_path_str)
        .expect(format!("Could not open file \"{}\"", in_path_str).as_str());
    let mut total_prior = 0usize;
    let mut group_bags: [HashSet<char>; 3] = [HashSet::from([]), HashSet::from([]),HashSet::from([])];
    for (i, line_result) in lines.enumerate() {
        let line = line_result.expect("Couldn't read line");
        group_bags[i % 3] = line.chars().collect();
        
        if i % 3 == 2 {
            let elf1and2bags = group_bags[0].intersection(&group_bags[1]).map(|it| it.clone()).collect();
            let shared_category: Vec<&char> = group_bags[2].intersection(&elf1and2bags).collect();
            total_prior += char_to_priority(shared_category[0]) as usize;
        }
    }
    println!("Total priority: {}", total_prior)
}

fn read_lines<P>(in_file_path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(in_file_path)?;
    Ok(BufReader::new(file).lines())
}

fn char_to_priority(c: &char) -> u8 {
        match c {
            'a'..='z'=> *c as u8 - 'a' as u8 + 1, 
            'A'..='Z' => *c as u8 - 'A' as u8 + 27u8,
            _ => panic!("Backpack sections don't have ascii characters in them"),
        }
}
