use std::{fs, cmp::{max, min}};

fn is_symbol_around(three_lines: [&str; 3], start: isize, end: usize, length: usize) -> bool {
    let (min, max) = (max(start - 1, 0) as usize, min(end + 1, length - 1));
    three_lines.iter().any(|line| {
        !line.is_empty() && line[min..=max]
            .chars()
            .any(|c| !c.is_numeric() && c != '.')
    })

}

fn main() {
    // TODO : Not use read to string :(
    let input = fs::read_to_string("../inputs/input").expect("Read error");
    let mut iter_lines = input.lines().peekable();
    let mut sum: usize = 0;

    // TODO : https://stackoverflow.com/questions/62186871/how-to-correctly-use-peek-in-rust

    let mut start: isize  = 0;
    let mut is_numbering = false;
    let mut previous_line = "";
    while let Some(line) = iter_lines.next() {
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if !is_numbering {
                    start = i as isize;
                    is_numbering = true;
                }
            } else if is_numbering {
                let three_lines: [&str; 3] = [previous_line, line, iter_lines.peek().unwrap_or(&"")];
                if is_symbol_around(three_lines, start, i-1, line.len()) {
                    sum += &line[start as usize..i].parse::<usize>().unwrap();
                }
                is_numbering = false;
            }
        }
        previous_line = &line;
    }
    println!("{sum:}");
}