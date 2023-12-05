use std::{fs, collections::VecDeque};

fn main() {
    let input = fs::read_to_string("../inputs/exemple1").expect("Read error");
    let mut sum = 0; 
    let mut three_lines: VecDeque<&str> = VecDeque::from(input.lines().take(3).collect::<Vec<&str>>());
    three_lines.shrink_to_fit();
    print!("{}", three_lines.capacity())
    // for line in input.lines() {
    //     println!("{sum:?}");
    // }
}

