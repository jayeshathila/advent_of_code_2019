use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn count_fuel(mass: i32) -> i32 {
    return mass/3 - 2 
}

fn main() {
    let lines = lines_from_file("/Users/jayeshathila/personal_projects/advent_of_code/advent_of_code/aoc_inputs/01");
    let mut rv: i32 = 0;
    for line in lines {
        rv = rv + count_fuel(line.parse::<i32>().unwrap());
    }
    println!("{}",rv)
}
