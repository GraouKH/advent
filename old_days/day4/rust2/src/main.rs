
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Card {
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>
}

impl Card {
    fn count(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
    }
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let card_lists = s.split_once(":").unwrap().1.split_once("|").unwrap();

        let winning_numbers: HashSet<usize> = card_lists.0
            .split_whitespace()
            .map(|r| r.trim().parse::<usize>().unwrap())
            .collect();

        let numbers: HashSet<usize> = card_lists.1
            .split_whitespace()
            .map(|r| r.trim().parse::<usize>().unwrap())
            .collect();
        Ok(Card { winning_numbers, numbers })
    }
}

fn main() {
    let file = File::open("../inputs/input").expect("Read error");
    let input_lines = BufReader::new(file).lines();
    let mut points: Vec<usize> = Vec::new();
    let mut counts: Vec<usize> = vec![1];

    for (i, line) in input_lines.enumerate() {
        let card_count: usize = line.unwrap().parse::<Card>().unwrap().count();
        points.push(card_count);
        while counts.len() < i + card_count + 1 {
            counts.push(1);
        }
        for j in 0..card_count {
            counts[j + 1 + i] += counts[i];
        }
    }
    let sum: usize = counts[..points.len()].iter().sum();
    println!("{}", sum);
}
