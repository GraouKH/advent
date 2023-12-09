
use std::fs::{File, self};
use std::io::{BufRead, BufReader};

struct Cog {
    range: Vec<usize>,
    modifier: Vec<usize>
}
struct Almanac {
    seeds: Vec<usize>,
    cogs: Vec<Cog>
}

impl Almanac {
    fn points(&self) -> usize {
        let count = self.winning_numbers.intersection(&self.numbers).count();
        if count > 0 { 1 << count - 1 } else { 0 }
    }
}

impl FromStr for Almanac {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let card_lists = s.split_once(":").unwrap().1.split_once("|").unwrap();

        let seeds: Vec<usize> = card_lists.0
            .split_whitespace()
            .map(|r| r.trim().parse::<usize>().unwrap())
            .collect();

        let cogs: Vec<Cog> = card_lists.1
            .split_whitespace()
            .map(|r| r.trim().parse::<usize>().unwrap())
            .collect();
        Ok(Almanac { seeds, cogs })
    }
}

fn main() {
    let file = File::open("exemple").expect("Read error");
    let input_lines = fs::read_to_string("exemple").expect("Read error");
    
    let almanac = input_lines.parse::<Almanac>().unwrap();
    let mut locations: Vec<usize> = Vec::with_capacity(almanac.seeds.len());
    for seed in almanac.seeds {
        location.push(almanac.compute(seed))
    }
    // let seed_line = input_lines.next().unwrap().unwrap();
    // let seeds: Vec<usize> = seed_line.split_once(":").unwrap().1
    //                                  .split_whitespace()
    //                                  .map(|r| r.trim().parse::<usize>().unwrap())
    //                                  .collect();
    let location_min: usize = locations.iter().min(); 
    println!("{location_min:?}");
}
