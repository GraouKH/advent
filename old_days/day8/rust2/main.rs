use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};
use regex::Regex;
use num_integer::lcm;

fn lcm_of_vec(numbers: &Vec<u128>) -> u128 {
    if numbers.is_empty() {
        return 0;
    }
    let mut result = numbers[0];
    for &num in numbers.iter().skip(1) {
        result = lcm(result, num);
    }
    result
}
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
    let mut count: u128 = 0;
    let mut new_starts = Vec::with_capacity(starts.len());
    let mut cycles: Vec<u128> = Vec::new();
    while !starts.is_empty() {
        let turn = instructions[count as usize % number_instructions];
        count += 1;
        for current_node in &starts {
            let current_pair = nodes.get(current_node).unwrap();
            let new_node = match turn {
                'L' => &current_pair.0,
                'R' => &current_pair.1,
                _ => "",
            };
            if !new_node.ends_with('Z') {
                new_starts.push(new_node.to_string());
            } else {
                cycles.push(count);
            }
        }
        starts = new_starts.clone();
        new_starts.clear();
    }

    println!("result: {}", lcm_of_vec(&cycles));
}
