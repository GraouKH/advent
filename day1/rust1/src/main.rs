use std::{fs, char};

fn main() {
    let input = fs::read_to_string("../inputs/input1").expect("Read error");
    let mut sum = 0;
    for line in input.lines() {
        let nums: Vec<char> = line.chars().filter(|&c| c.is_numeric()).collect();
        sum += format!("{}{}", nums.first().unwrap(), nums.last().unwrap()).parse::<i32>().unwrap();
    }
    println!("{:?}", sum);
}
