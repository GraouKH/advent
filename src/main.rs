use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
use regex::Regex;

fn main() {
    let file = File::open("input").unwrap();
    let mut input_lines = BufReader::new(file).lines();

    let first_line = input_lines.next().expect("No input").expect("Read error");
    let instructions: Vec<char> = first_line.chars().collect();
    let number_instructions = instructions.len();
    input_lines.next();

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    let mut starts: Vec<String> = Vec::new();

    while let Some(line) = input_lines.next() {
        let re = Regex::new(r"^(\w+) = \((\w+), (\w+)\)$").expect("Regex compilation error");
        if let Some(n) = re.captures(&line.expect("Read error")) {
            let name = n[1].to_string();
            nodes.insert(name.clone(), (n[2].to_string(), n[3].to_string()));
            if name.ends_with('A') {
                starts.push(name);
            }
        }
    }

    println!("{}", starts.len());
    let mut count: usize = 0;
    let mut new_starts = Vec::with_capacity(starts.len());
    while starts.iter().any(|s| !s.ends_with('Z')) {
        let turn = instructions[count % number_instructions];
        for current_node in &starts {
            let current_pair = nodes.get(current_node).unwrap();
            let new_node = match turn {
                'L' => &current_pair.0,
                'R' => &current_pair.1,
                _ => "",
            };
            new_starts.push(new_node.to_string());
        }
        starts = new_starts.clone();
        new_starts.clear();
        count += 1;
        if count % 1000000 == 0 {
            println!("{}", count);
        }
    }
    println!("{}", count);
}
