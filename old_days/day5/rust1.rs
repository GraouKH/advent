
use std::{fs, str::FromStr};

struct Range {
    new_start: usize,
    range_start: usize,
    range_length: usize
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range: Vec<usize> = s.split_whitespace().map(|r| r.parse().unwrap()).collect();
        println!("{} {} {}", range[0], range[1], range[2]);
        Ok(Range { new_start: range[0], range_start: range[1], range_length: range[2]})
    }
}

struct Cog {
    ranges: Vec<Range>
}

impl Cog {
    fn compute(&self, mut x: usize) -> usize {
        for r in &self.ranges {
            if (r.range_start..=r.range_start + r.range_length).contains(&x) {
                x = (&r.new_start + &x) - &r.range_start ;
                break;
            } else {
                x = x;
            };
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
        let almanac_lines: Vec<&str> = s.split("\r\n\r\n").collect();
        let seed_line = almanac_lines[0];
        let seeds: Vec<usize> = seed_line.split_once(":").unwrap().1
            .split_whitespace()
            .map(|r| r.trim().parse::<usize>().unwrap())
            .collect();

        let mut cogs: Vec<Cog> = Vec::new();
        for cog in &almanac_lines[1..] {
            let mut ranges: Vec<Range> = Vec::new();
            for line in cog.split("\n").skip(1) {
                ranges.push(line.parse::<Range>().unwrap());
            }
            cogs.push(Cog { ranges});
        }

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
    let input = fs::read_to_string("input").expect("Read error");
    
    let almanac = input.parse::<Almanac>().unwrap();
    let locations: Vec<usize> = almanac.compute();
    
    let location_min: usize = *locations.iter().min().unwrap(); 
    println!("{location_min:?}");
}
