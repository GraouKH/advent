
use std::{fs, str::FromStr};

struct Range {
    new_start: usize,
    range_start: usize,
    range_length: usize
}

struct Cog {
    ranges: Vec<Range>
}

impl Cog {
    fn compute(&self, mut x: usize) -> usize {
        for r in &self.ranges {
            x = if (r.range_start..=r.range_start + r.range_length).contains(&x) {&r.new_start + &r.range_start + &r.range_length - &x} else {x};
            break;
        }
        x
    }
}
struct Almanac {
    seeds: Vec<usize>,
    cogs: Vec<Cog>
}

impl FromStr for Almanac {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut almanac_lines: Vec<&str> = s.split("\r\n\r\n").collect();
        let seed_line = almanac_lines[0];
        let seeds: Vec<usize> = seed_line.split_once(":").unwrap().1
            .split_whitespace()
            .map(|r| r.trim().parse::<usize>().unwrap())
            .collect();

        let cogs: Vec<Cog> = Vec::new();
        almanac_lines[1..]
        println!("{s:?}");
        // let cogs: Vec<Cog> = card_lists.1
        //     .split_whitespace()
        //     .map(|r| r.trim().parse::<usize>().unwrap())
        //     .collect();
        Ok(Almanac { seeds, cogs })
    }
}

impl Almanac {
    fn compute(&self) -> Vec<usize> {
        let mut location = Vec::with_capacity(self.seeds.len());
        for s in &self.seeds {
            let mut l = *s;
            for c in &self.cogs{
                l = c.compute(l);
            }
            location.push(l);
        }
        location
    }
}
fn main() {
    let input = fs::read_to_string("exemple").expect("Read error");
    
    let almanac = input.parse::<Almanac>().unwrap();
    let locations: Vec<usize> = almanac.compute();
    
    let location_min: usize = *locations.iter().min().unwrap(); 
    println!("{location_min:?}");
}
