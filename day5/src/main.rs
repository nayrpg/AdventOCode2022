use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines,},
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

#[derive(Debug)]
struct ContainerConfig {
    _num_of_stacks: usize,
    stacks: Vec<Vec<char>>,
}
fn part1(in_path_str: String) {
    let lines = read_lines(&in_path_str)
        .expect(format!("Could not open file \"{}\"", in_path_str).as_str());
    let mut read_start_map = false;
    let mut containers: Vec<Vec<String>> = Vec::new();
    let mut cont_config: ContainerConfig = ContainerConfig { _num_of_stacks: 0, stacks: Vec::new() };
    // let mut crate_config: Vec<Vec<>>;
    for line_result in lines {
        let line = line_result.expect("Couldn't read line");
        if line == "" {
            if !read_start_map {
                cont_config = ContainerConfig::new(&containers);
            }
            read_start_map ^= true;
            continue
        }
        if !read_start_map {
            containers.push(line.split(' ').map(|cont| String::from(cont)).collect());
        } else {
            cont_config.do_command(&line);
        }
    }
    println!("Containers: {:?}", cont_config);
    let mut final_str: Vec<char> = Vec::new();
    for stack in cont_config.stacks {
        final_str.push(*stack.last().unwrap());
    }
    println!("{}", final_str.iter().collect::<String>())
}
fn part2(in_path_str: String) {
    let lines = read_lines(&in_path_str)
        .expect(format!("Could not open file \"{}\"", in_path_str).as_str());
    let mut read_start_map = false;
    let mut containers: Vec<Vec<String>> = Vec::new();
    let mut cont_config: ContainerConfig = ContainerConfig { _num_of_stacks: 0, stacks: Vec::new() };
    // let mut crate_config: Vec<Vec<>>;
    for line_result in lines {
        let line = line_result.expect("Couldn't read line");
        if line == "" {
            if !read_start_map {
                cont_config = ContainerConfig::new(&containers);
            }
            read_start_map ^= true;
            continue
        }
        if !read_start_map {
            containers.push(line.split(' ').map(|cont| String::from(cont)).collect());
        } else {
            cont_config.do_command9001(&line);
        }
    }
    println!("Containers: {:?}", cont_config);
    let mut final_str: Vec<char> = Vec::new();
    for stack in cont_config.stacks {
        final_str.push(*stack.last().unwrap());
    }
    println!("{}", final_str.iter().collect::<String>())
}
impl ContainerConfig {
    fn new(containers: &Vec<Vec<String>>) -> ContainerConfig {
        let (footer_line, containers) = containers.split_last().expect("Container config empty");
        let stackcount = ContainerConfig::get_stack_count(&footer_line);
        ContainerConfig {
            stacks: ContainerConfig::parse(&containers[..containers.len()], &stackcount),
            _num_of_stacks: stackcount,
        }
    }
    fn parse(containers_strs: &[Vec<String>], stack_count: &usize) -> Vec<Vec<char>> {
        let mut config = Vec::new();
        for _ in 0..*stack_count {
            config.push(Vec::new())
        }
        for level in containers_strs {
            let mut level_index = 0usize;
            for stack_i in 0..*stack_count {
                let cur_part = &level[level_index];
                if cur_part == "" {
                    level_index += 3;
                } else {
                    config[stack_i].push(cur_part.chars().nth(1).unwrap());
                }
                level_index += 1;
            }
        }
        for i in 0..*stack_count {
            for j in 0..config[i].len()/2 {
                let tmp = config[i][j];
                let swap_idx = config[i].len()-1-j;
                config[i][j] = config[i][swap_idx];
                config[i][swap_idx] = tmp;
            }
        }
        config
    }

    fn do_command (&mut self, command: &str) {
        let mut command_str = command.split(' ');
        let move_count: usize = command_str.nth(1).unwrap().parse().unwrap();
        let from_stack: usize = command_str.nth(1).unwrap().parse().unwrap();
        let to_stack: usize = command_str.nth(1).unwrap().parse().unwrap();

        for _ in 0..move_count {
           let lifted = self.stacks[from_stack-1].pop().unwrap(); 
           self.stacks[to_stack-1].push(lifted);
        }
    }
    fn do_command9001 (&mut self, command: &str) {
        let mut command_str = command.split(' ');
        let move_count: usize = command_str.nth(1).unwrap().parse().unwrap();
        let from_stack: usize = command_str.nth(1).unwrap().parse().unwrap();
        let to_stack: usize = command_str.nth(1).unwrap().parse().unwrap();


        let mut lifted: Vec<char> = Vec::new();
        for _ in 0..move_count {
            lifted.push(self.stacks[from_stack-1].pop().unwrap());
        }
        for _ in 0..move_count {
            self.stacks[to_stack-1].push(lifted.pop().unwrap());
        }
    }
    fn get_stack_count(footer_line: &Vec<String>) -> usize {
        footer_line.len() / 3usize
    }
}
fn read_lines<P>(in_file_path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(in_file_path)?;
    Ok(BufReader::new(file).lines())
}
