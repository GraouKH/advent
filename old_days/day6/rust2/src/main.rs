use std::fs;

fn description_to_number(description: &str) -> usize {
    let description_words: Vec<&str> = description.split_whitespace().collect();
    let description_numbers: &str = &description_words[1..].concat();
    description_numbers.parse().unwrap()
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
    let time: usize = description_to_number(time_str);
    let distance: usize = description_to_number(distance_str);

    let time_distances = all_distances(time);
    let sum = time_distances.iter().filter(|&&d| d > distance).count();

    println!("{sum:?}");
}
