
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>
}

impl Card {
    fn points(&self) -> usize {
        let count = self.winning_numbers.intersection(&self.numbers).count();
        if count > 0 { 1 << count - 1 } else { 0 }
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
    let mut sum: usize = 0;

    for line in input_lines {
        let card: Card = line.unwrap().parse().unwrap();
        sum += card.points();
    }
    println!("{sum:}");
}
