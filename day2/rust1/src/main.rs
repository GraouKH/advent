use std::str::FromStr;
use std::fs;

#[derive(Debug, Copy, Clone)]
enum Color {
    Blue(u32),
    Red(u32),
    Green(u32),
}

struct ColorLimit {
    blue: u32,
    red: u32,
    green: u32,
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();

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

        Ok(Round {colors: colors})
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn is_valid(&self, limit: &ColorLimit) -> bool {
        for round in &self.rounds {
            for color in &round.colors {
                match color {
                    Color::Blue(value) => if value > &limit.blue { return false },
                    Color::Red(value) => if value > &limit.red { return false },
                    Color::Green(value) => if value > &limit.green { return false }
                } 
            }
        }
        true
    }
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();

        if parts.len() != 2 {
            return Err("Invalid game format");
        }

        let id: u32 = parts[0][5..].trim().parse().map_err(|_| "Invalid game ID")?;
        let rounds: Vec<Round> = parts[1]
            .split(';')
            .map(|r| r.parse())
            .collect::<Result<_, _>>()?;

        Ok(Game { id, rounds })
    }

}

fn main() {
    let input = fs::read_to_string("../inputs/exemple1").expect("Read error");
    let mut sum = 0;
    let limit = ColorLimit { red: 12, green: 13, blue:14 };
    for line in input.lines() {
        let game: Game = line.parse().unwrap();
        if game.is_valid(&limit) {
            sum += game.id;
        }
    }
    println!("{:?}", sum);
}
