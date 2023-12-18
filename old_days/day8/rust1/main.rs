use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
use regex::Regex;

fn main() {
    let file = File::open("input").expect("Read error");
    let mut input_lines = BufReader::new(file).lines();

    let first_line = input_lines.next().unwrap().unwrap();
    let instructions: Vec<char> = first_line.chars().collect();
    let number_instructions = instructions.len();
    input_lines.next();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    while let Some(line) = input_lines.next() {
        let re = Regex::new(r"^(\w+) = \((\w+), (\w+)\)$").unwrap();
        if let Some(n) = re.captures(&line.unwrap()) {
            let name = n[1].to_string();
            let left = n[2].to_string();
            let right = n[3].to_string();
            nodes.insert(name.clone(), (left, right));
        }
    }
    
    let mut current_node= "AAA";
    let mut count: usize = 0;
    while current_node != "ZZZ" {
        let turn = instructions[count % number_instructions];
        let current_pair = nodes.get(current_node).unwrap();
        current_node = match turn {
            'L' => &current_pair.0,
            'R' => &current_pair.1,
            _ => "",
        };
        count += 1;
    }
    println!("{count}");
}
