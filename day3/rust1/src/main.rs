use std::{fs::File, cmp::{max, min}};
use std::io::{BufRead, BufReader};

fn is_symbol_around(previous_line: &str, current_line: &str, next_line: &str, start: isize, end: usize) -> bool {
    let (min, max) = (max(start - 1, 0) as usize, min(end + 1, current_line.len() - 1));
    let (start_char, end_char) = (current_line.chars().nth(min).unwrap(), current_line.chars().nth(max).unwrap());
    previous_line[min..=max].chars().any(|c| !c.is_numeric() && c != '.') && next_line[min..=max].chars().any(|c| !c.is_numeric() && c != '.') && !start_char.is_numeric() && start_char != '.' && !end_char.is_numeric() && end_char != '.' 
}

fn main() {
    let file = File::open("../inputs/input").expect("Read error");
    let mut input_lines = BufReader::new(file).lines();
    let mut sum: usize = 0;

    let mut start: isize  = 0;
    let mut is_numbering = false;
    let mut previous_line = String::default();
    let mut current_line = input_lines.next().unwrap().unwrap();
    let mut next_line = input_lines.next().unwrap().unwrap();
    while let Some(line) = input_lines.next() {
        for (i, c) in current_line.chars().enumerate() {
            if c.is_numeric() {
                if !is_numbering {
                    start = i as isize;
                    is_numbering = true;
                }
            } else if is_numbering {
                if is_symbol_around(&previous_line, &current_line, &next_line, start, i-1) {
                    sum += &current_line[start as usize..i].parse::<usize>().unwrap();
                }
                is_numbering = false;
            }
        }
        previous_line = current_line;
        current_line = next_line;
        next_line = line.unwrap();
    }
    println!("{sum:}");
}