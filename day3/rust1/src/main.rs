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
    let input = fs::read_to_string("../inputs/exemple1").expect("Read error");
    let mut sum: usize = 0;

    // TODO : Use peek to check next lines
    let mut lines: [&str; 3] = Default::default();
    let mut iter_lines = input.lines();
    lines[0] = iter_lines.next().unwrap();
    lines[1] = iter_lines.next().unwrap();
    let length_line = lines[0].len();

    let mut start: isize  = 0;
    let mut is_numbering = false;
    let mut current_index = 0;
    let mut new_index = 2;
    for line in iter_lines {
        for (i, c) in lines[current_index].chars().enumerate() {
            if c.is_numeric() {
                if !is_numbering {
                    start = i as isize;
                    is_numbering = true;
                }
            } else if is_numbering {
                if is_symbol_around(lines, start, i-1, length_line) {
                    println!("{}, {}", &lines[current_index][start as usize..i], &line);
                    sum += &lines[current_index][start as usize..i].parse::<usize>().unwrap();
                }
                is_numbering = false;
            }
        }
        lines[new_index] = line;
        current_index = (current_index + 1) % 3;
        new_index = (new_index + 1) % 3;
    }
    println!("{sum:}");
}