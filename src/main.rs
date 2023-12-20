use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("input").expect("Read error");
    let mut input_lines = BufReader::new(file).lines();

    
}
