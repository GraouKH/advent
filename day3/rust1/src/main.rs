use std::{fs, cmp::{max, min}};

fn is_symbol_around(three_lines: [&str; 3], start: isize, end: usize, length: usize) -> bool {
    let (min, max) = (max(start - 1, 0) as usize, min(end, length - 1));
    println!("{start} {end} {min} {max}");
    three_lines.iter().any(|line| {
        !line.is_empty() && line[min..=max]
            .chars()
            .any(|c| !c.is_numeric() && c != '.')
    })

}

fn main() {
    let input = fs::read_to_string("../inputs/exemple1").expect("Read error");
    let mut sum: usize = 0;

    let mut lines: [&str; 3] = Default::default();
    lines[0] = input.lines().nth(0).unwrap();
    let length_line = lines[0].len();

    println!("{length_line}");
    let mut start: isize  = 0;
    let mut is_numbering = false;
    let mut current_index = 1;
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if !is_numbering {
                    start = i as isize;
                    is_numbering = true;
                }
            } else if is_numbering {
                if is_symbol_around(lines, start, i-1, length_line) {
                    sum += line[start as usize..i].parse::<usize>().unwrap();
                }
                is_numbering = false;
            }
        }
        lines[current_index] = line;
        current_index = (current_index + 1) % 3;
    }
    println!("{sum:}");
}