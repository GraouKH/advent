use std::{fs, iter::zip};

fn description_to_numbers(description: &str) -> Vec<usize> {
    let description_words: Vec<&str> = description.split_whitespace().collect();
    let description_numbers: &[&str] = &description_words[1..];
    let parsed_numbers: Result<Vec<usize>, _> = description_numbers.iter().map(|s| s.parse()).collect();
    parsed_numbers.unwrap()
}

fn all_distances(time: usize) -> Vec<usize> {
    let mut distances: Vec<usize> = Vec::with_capacity(time - 1);
    for t in 1..time - 1 {
        distances.push(t * (time - t));
    }
    distances
}

fn main() {
    let input = fs::read_to_string("../inputs/input").expect("Read error");
    let (time_str, distance_str) = input.split_once("\n").unwrap();
    let times: Vec<usize> = description_to_numbers(time_str);
    let distances: Vec<usize> = description_to_numbers(distance_str);

    let mut sum = 1;
    for (&time, &distance) in zip(&times, &distances) {
        let time_distances = all_distances(time);
        sum *= time_distances.iter().filter(|&&d| d > distance).count();
    } 

    println!("{sum:?}");
}
