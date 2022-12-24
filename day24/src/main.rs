use std::{collections::HashSet, fmt::Display};

type Coordinate = (usize, usize);

#[derive(Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => f.write_str("^"),
            Direction::South => f.write_str("v"),
            Direction::East => f.write_str(">"),
            Direction::West => f.write_str("<"),
        }
    }
}

impl Direction {
    fn from(character: u8) -> Option<Direction> {
        match character {
            b'^' => Some(Direction::North),
            b'v' => Some(Direction::South),
            b'>' => Some(Direction::East),
            b'<' => Some(Direction::West),
            b'.' => None,
            b'#' => None,
            _ => unreachable!("Unknown character {}", char::from(character)),
        }
    }

    fn shift(&self, direction: Coordinate) -> Coordinate {
        match self {
            Direction::North => (direction.0, direction.1 - 1),
            Direction::South => (direction.0, direction.1 + 1),
            Direction::East => (direction.0 + 1, direction.1),
            Direction::West => (direction.0 - 1, direction.1),
        }
    }
}

struct Blizzard {
    direction: Direction,
    position: Coordinate,
}

impl Blizzard {
    fn from(direction: u8, position: Coordinate) -> Option<Blizzard> {
        if let Some(direction) = Direction::from(direction) {
            Some(Blizzard {
                direction,
                position,
            })
        } else {
            None
        }
    }

    fn shift(&self) -> Coordinate {
        self.direction.shift(self.position)
    }
}

#[derive(Default)]
struct Map {
    blizzards: Vec<Blizzard>,
    occupied: HashSet<Coordinate>,
    width: usize,
    height: usize,
}

impl Map {
    fn from(string: &str) -> Map {
        let mut map = Map::default();
        for (y, line) in string.trim().split('\n').enumerate() {
            map.height += 1;
            map.width = line.trim().as_bytes().len();
            for (x, character) in line.trim().bytes().enumerate() {
                if let Some(blizzard) = Blizzard::from(character, (x, y)) {
                    map.blizzards.push(blizzard);
                }
            }
        }
        map.calculate_occupied();
        map
    }

    fn calculate_occupied(&mut self) {
        self.occupied = self
            .blizzards
            .iter()
            .map(|blizzard| blizzard.position)
            .collect();
    }

    fn step(&self) -> Map {
        let mut other = Map::default();
        other.width = self.width;
        other.height = self.height;
        other.blizzards.reserve_exact(self.blizzards.len());
        for blizzard in &self.blizzards {
            let mut position = blizzard.shift();
            if position.0 >= self.width - 1 {
                position.0 = 1;
            }
            if position.0 <= 0 {
                position.0 = self.width - 2;
            }
            if position.1 >= self.height - 1 {
                position.1 = 1;
            }
            if position.1 <= 0 {
                position.1 = self.height - 2;
            }
            other.blizzards.push(Blizzard {
                position,
                direction: blizzard.direction.clone(),
            });
        }
        other.calculate_occupied();
        other
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let blizzards: Vec<&Blizzard> = self
                    .blizzards
                    .iter()
                    .filter(|blizzard| blizzard.position == (x, y))
                    .collect();
                match blizzards.len() {
                    0 => print!("."),
                    1 => print!("{}", blizzards[0].direction),
                    _ => print!("{}", blizzards.len()),
                }
            }
            println!("");
        }
        println!("\n");
    }
}

fn main() {
    let file = include_str!("../input.txt");
    let mut map = Map::from(file);
    map.print();

    for i in 0..10 {
        map = map.step();
        map.print();
    }
}
