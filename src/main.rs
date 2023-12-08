
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("exemple").expect("Read error");
    let mut input_lines = BufReader::new(file).lines();

    let seed_line = input_lines.next().unwrap().unwrap();
    let seeds: Vec<usize> = seed_line.split_once(":").unwrap().1
                                     .split_whitespace()
                                     .map(|r| r.trim().parse::<usize>().unwrap())
                                     .collect();
    
    println!("{seeds:?}");
    // for (i, line) in input_lines.enumerate() {
    // TODO: Split par /n/n pour récup chaque rouage, chaque rouage appelle le suivant
    // }
    // println!("{}", sum);
}
