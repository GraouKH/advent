use std::{fs, str::FromStr};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Range {
    range_start: usize,
    new_start: usize,
    range_end: usize
}

impl FromStr for Range {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range: Vec<usize> = s.split_whitespace().map(|r| r.parse().unwrap()).collect();
        Ok(Range { new_start: range[0], range_start: range[1], range_end: range[1] + range[2]})
    }
}

struct Cog {
    ranges: Vec<Range>
}

struct Almanac {
    seeds: Vec<Range>,
    cogs: Vec<Cog>
}

impl FromStr for Almanac {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let almanac_lines: Vec<&str> = s.split("\r\n\r\n").collect();
        let seed_line = almanac_lines[0];
        let mut iter_seeds  = seed_line.split_once(":").unwrap().1
            .split_whitespace()
            .map(|r| r.trim().parse::<usize>().unwrap());

        
        // let mut seeds: HashSet<usize> = HashSet::new();
        let mut seeds: Vec<Range> = Vec::new();
        while let Some(seed_start) = iter_seeds.next() { 
            seeds.push(Range{ new_start: 0, range_start: seed_start, range_end: iter_seeds.next().unwrap() + seed_start});
        }

        let mut cogs: Vec<Cog> = Vec::new();
        for cog in &almanac_lines[1..] {
            let mut ranges: Vec<Range> = Vec::new();
            for line in cog.split("\n").skip(1) {
                ranges.push(line.parse::<Range>().unwrap());
            }
            cogs.push(Cog { ranges });
        }

        Ok(Almanac { seeds, cogs })
    }
}

impl Almanac {
    fn compute(&self)  -> usize {
        let mut min = usize::MAX;
        for s in &self.seeds {
            let mut i = s.range_start;
            let mut diff = s.range_end - i;
            while i < s.range_end {
                'cogs: for cog in &self.cogs {
                    for range in &cog.ranges {
                        if i < range.range_start || i > range.range_end {
                            continue;
                        }
                        if range.range_end - i < diff {
                            diff = range.range_end - i;
                        }
                        i = i + range.new_start - range.range_start;
                        continue 'cogs;
                    }
                }
                if i < min {
                    min = i;
                }
                i += diff + 1;
            }
        }
        min
    }

    fn cogs_full_range(&mut self) {
        for cog in &mut self.cogs {
            let mut new_ranges: Vec<Range> = Vec::new();
            cog.ranges.sort();
            let mut min_range: usize = 0;
            for range in &cog.ranges {
                if min_range < range.range_start {
                    new_ranges.push(Range{ new_start: min_range, range_start: min_range, range_end: range.range_start});
                }
                new_ranges.push(*range);
                min_range = range.range_end;
            }
            cog.ranges = new_ranges;
            
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("Read error");
    let mut almanac = input.parse::<Almanac>().unwrap();
    println!("input parsed");
    almanac.cogs_full_range();
    println!("cogs full range");
    let min = almanac.compute();
    println!("{min}");
}
