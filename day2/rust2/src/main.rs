use std::str::FromStr;
use std::fs;

#[derive(Debug, Copy, Clone)]
enum Color {
    Blue(u32),
    Red(u32),
    Green(u32),
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() != 2 {
            return Err("Invalid color format");
        }

        let count: u32 = parts[0].parse().map_err(|_| "Invalid count")?;
        let color = match parts[1] {
            "blue" => Color::Blue(count),
            "red" => Color::Red(count),
            "green" => Color::Green(count),
            _ => return Err("Unknown color"),
        };

        Ok(color)
    }
}

#[derive(Debug)]
struct Round {
    colors: Vec<Color>,
}

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colors: Vec<Color> = s
            .split(',')
            .map(|color_str| color_str.parse())
            .collect::<Result<_, _>>()?;

        Ok(Round {colors})
    }
}

#[derive(Debug)]
struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn min_cubes(&self) -> u32 {
        let (mut min_red, mut min_blue, mut min_green) = (0, 0, 0);
        for round in &self.rounds {
           for color in &round.colors {
                match color {
                    Color::Blue(value) => if value > &min_blue { min_blue = *value },
                    Color::Red(value) => if value > &min_red { min_red = *value },
                    Color::Green(value) => if value > &min_green { min_green = *value }
                }
           }
        }
        min_red * min_blue * min_green
    }
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        if parts.len() != 2 {
            return Err("Invalid game format");
        }

        let rounds: Vec<Round> = parts[1]
            .split(';')
            .map(|r| r.parse())
            .collect::<Result<_, _>>()?;

        Ok(Game { rounds })
    }

}

fn main() {
    let input = fs::read_to_string("../inputs/input1").expect("Read error");
    let mut sum = 0;
    for line in input.lines() {
        let game: Game = line.parse().unwrap();
        sum += game.min_cubes();
    }
    println!("{sum:?}");
}
