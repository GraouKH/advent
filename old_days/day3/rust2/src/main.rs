
use std::fs::File;
use std::io::{BufRead, BufReader};

fn check_line(line: &str, index: isize) -> Vec<usize> {
    let mut start: isize  = 0;
    let mut is_numbering = false;
    let mut numbers = Vec::new();
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            if !is_numbering {
                start = i as isize;
                is_numbering = true;
            }
        } else if is_numbering {
            if (start - 1..=i as isize).contains(&index) {
                numbers.push(line[start as usize..i].parse::<usize>().unwrap());
            }
            is_numbering = false;
        }
    }
    if is_numbering && (start - 1..line.len() as isize).contains(&index) {
        numbers.push(line[start as usize..].parse::<usize>().unwrap());
    }
    numbers
}

fn parse_side_number<T: Iterator<Item = char>>(chars: T) -> String {
    chars.take_while(|&c| c.is_numeric()).collect::<String>()
}

fn check_lines(lines: &[String], len: usize) -> usize {
    let mut sum: usize = 0;
    let big_line = lines.join("");
    for (i, c) in big_line[len..len*2].chars().enumerate() {
        if c == '*' {
            let mut numbers: Vec<usize> = Vec::new();
            let left: String = parse_side_number(big_line[len..len+i].chars().rev()); 
            if !left.is_empty() {
                numbers.push(left.chars().rev().collect::<String>().parse::<usize>().unwrap());
            }
            let right: String = parse_side_number(big_line[len+i+1..len*2].chars()); 
            if !right.is_empty() {
                numbers.push(right.parse::<usize>().unwrap());
            }
            numbers.append(&mut check_line(&big_line[..len], i as isize));
            numbers.append(&mut check_line(&big_line[len*2..], i as isize));
            if numbers.len() == 2 {
                sum += numbers[0] * numbers[1];
            }
        }
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
    sum += check_lines(&lines, line_length);
    lines.remove(0);
    sum += check_lines(&lines, line_length);
    println!("{sum:}");
}
