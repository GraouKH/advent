use std::{fs::File, io::{BufReader, BufRead}, str::FromStr};

#[derive(Clone, Copy)]
enum Tile {
    Vertical,
    Horizontal,
    SouthWest,
    SouthEast,
    NorthWest,
    NorthEast,
    Start,
    Ground
}

impl FromStr for Tile {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tile = match s.chars().next().unwrap() {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            'J' => Tile::NorthWest,
            'L' => Tile::NorthEast,
            'S' => Tile::Start,
            _ => Tile::Ground,
        };

        Ok(tile)
    }
}

impl Tile {
    fn next_direction(self, d: Direction) -> Direction {
        match d {
            Direction::North => {
                match self {
                    Vertical => Direction::South,
                    NorthWest => Direction::West,
                    NorthEast => Direction::East,
                    _ => Direction::NoDirection
                }
            }
            Direction::South => {
                match self {
                    Vertical => Direction::North,
                    SouthWest => Direction::West,
                    SouthEast => Direction::East,
                    _ => Direction::NoDirection
                }
            }
            Direction::West => {
                match self {
                    Horizontal => Direction::East,
                    NorthWest => Direction::North,
                    SouthWest => Direction::South,
                    _ => Direction::NoDirection
                }
            }
            Direction::East => {
                match self {
                    Horizontal => Direction::West,
                    NorthEast => Direction::North,
                    SouthEast => Direction::South,
                    _ => Direction::NoDirection
                }
            }
            Direction::NoDirection => {
                Direction::NoDirection
            }
        }
    }
}

fn parse_tiles(s: &str) -> Vec<Tile> {
    s.chars().map(|c| c.to_string().parse::<Tile>().unwrap()).collect::<Vec<Tile>>()
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
    NoDirection,
}

impl Direction {
    fn next_tile(self, idx: (isize, isize)) -> (isize, isize) {
        match self {
            North => (idx.0 - 1, idx.1),
            South => (idx.0 + 1, idx.1),
            West => (idx.0, idx.1 - 1),
            East => (idx.0, idx.1 + 1),
            NoDirection => idx
        }
    }
}

fn next_tile(tiles: &Vec<Vec<Tile>>, tile: &mut Tile, direction: &mut Direction, idx: &mut (isize, isize)) -> bool {
    let new_idx = direction.next_tile(*idx);
    if new_idx.0 >= 0 && new_idx.1 >= 0 && new_idx.0 < tiles.len() as isize && new_idx.1 < tiles[0].len() as isize {
        let new_tile = &tiles[new_idx.0 as usize][new_idx.1 as usize];
        let new_direction = tile.next_direction(*direction);
        if let Direction::NoDirection = new_direction {
            return false;
        } else {
            *tile = *new_tile;
            *direction = new_direction;
            *idx = new_idx;
            return true;
        }
    }
    false
}

fn main() {
    let file = File::open("exemple").expect("Read error");
    let mut input_lines = BufReader::new(file).lines();
    let mut farthest_point: isize = 0;
    let mut parsed_tiles: Vec<Vec<Tile>> = Vec::new();
    while let Some(line) = input_lines.next() {
        parsed_tiles.push(parse_tiles(&line.unwrap()));
    }
    let mut direction: Direction = Direction::NoDirection;
    for (i, row) in parsed_tiles.iter().enumerate() {
        for (j, mut tile) in row.iter().enumerate() {
            if next_tile(&parsed_tiles, &mut tile, &mut direction, &mut (i as isize, j as isize)) {
                println!("next");
            }
        }
    }
    println!("{farthest_point}");
}
