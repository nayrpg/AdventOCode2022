use std::{env, path::Path};

fn main() {
    let mut arg_iter = env::args();
    arg_iter.next();
    if let Some(part_str) = arg_iter.next() {
        if let Ok(part) = part_str.parse::<usize>() {
            let in_str = match arg_iter.next() {
                Some(string) => string,
                _ => String::from("input"),
            };
            match part {
                1 => part1(in_str),
                2 => part2(in_str),
                _ => println!("There is no part {}", part),
            }
        } else {
            println!("Day given is not a positive integer")
        }
    } else {
        println!("No day one part given!")
    }
}
fn part1(in_path_str: String) {
    let _in_path = Path::new(&in_path_str);
    println!("{}", in_path_str)
}
fn part2(_path: String) {
    println!("part2 not implemented")
}