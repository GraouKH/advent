use std::fs::File;
use std::io::{BufRead, BufReader};

const LOOK_AROUND:  = 10;

fn check_lines(previous_line: &str, current_line: &str, next_line: &str) -> usize {
    let mut sum: usize = 0;
    let numbers: [usize; 2] = [0, 0];
    for (i, c) in current_line.chars().enumerate() {
        if c == '*' {
             
        };
    }
    sum
}

fn main() {
    let file = File::open("../inputs/input").expect("Read error");
    let mut input_lines = BufReader::new(file).lines();
    let mut sum: usize = 0;

    let mut previous_line = String::default();
    let mut current_line = input_lines.next().unwrap().unwrap();
    let mut next_line = input_lines.next().unwrap().unwrap();

    for line in input_lines {
        sum += check_lines(&previous_line, &current_line, &next_line);
        previous_line = current_line;
        current_line = next_line;
        next_line = line.unwrap();
    }

    sum += check_lines(&previous_line, &current_line, &next_line);
    sum += check_lines(&current_line, &next_line, "");
    println!("{sum:}");
}