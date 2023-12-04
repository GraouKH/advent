use std::{fs, char};

fn main() {
    let input = fs::read_to_string("../inputs/exemple").expect("Read error");
    for line in input.lines() {
        println!("{}", line)
    }
}
