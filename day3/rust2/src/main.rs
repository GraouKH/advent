
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_asterisk_around(previous_line: &str, current_line: &str, next_line: &str, start: isize, end: usize) -> bool {
    let min = max(start - 1, 0) as usize;
    let (start_char, end_char) = (current_line.chars().nth(min).unwrap(), current_line.chars().nth(end + 1).unwrap());
    (!previous_line.is_empty() && previous_line[min..=end + 1].chars().any(|c| !c.is_numeric() && c == '*')) || 
    (!next_line.is_empty() && next_line[min..=end + 1].chars().any(|c| !c.is_numeric() && c == '*')) ||
    (!start_char.is_numeric() && start_char == '*') || (!end_char.is_numeric() && end_char == '*')
}

fn check_lines(lines: &Vec<String>, len: usize) -> usize {
    let mut sum: usize = 0;
    let mut start: isize  = 0;
    let mut is_numbering = false;
    let big_line = lines.join("");
    for (i, c) in big_line[len..len*2].chars().enumerate() {
        if c.is_numeric() {
            if !is_numbering {
                start = i as isize;
                is_numbering = true;
            }
        } else if is_numbering {
            if is_asterisk_around(previous_line, current_line, next_line, start, i-1) {
                sum += current_line[start as usize..i].parse::<usize>().unwrap();
            }
            is_numbering = false;
        }
    }
    // Check when number is at the end of the line
    if is_numbering && is_asterisk_around(previous_line, current_line, next_line, start, current_line.len() - 2) {
        sum += current_line[start as usize..current_line.len()].parse::<usize>().unwrap();
    }
    sum
}

fn main() {
    let file = File::open("../inputs/input").expect("Read error");
    let mut input_lines = BufReader::new(file).lines();
    let mut sum: usize = 0;

    let mut lines: Vec<String> = Vec::with_capacity(3);
    let first_line: String = input_lines.next().unwrap().unwrap();
    let line_length = first_line.len();

    lines.push(String::default());
    lines.push(first_line);
    lines.push(input_lines.next().unwrap().unwrap());

    for line in input_lines {
        sum += check_lines(&lines, line_length);
        lines.remove(0);
        lines.push(line.unwrap());
    }

    // check the two last lines as well
    sum += check_lines(&previous_line, &current_line, &next_line);
    sum += check_lines(&current_line, &next_line, "");
    println!("{sum:}");
}
