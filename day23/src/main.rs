use std::{
    collections::{HashMap, HashSet},
};

type Coordinate = (i32, i32);

const ROUNDS: usize = 10;

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn is_clear(&self, elf: &Coordinate, map: &HashSet<Coordinate>) -> bool {
        let to_check = match &self {
            Direction::North => [(-1, -1), (0, -1), (1, -1)],
            Direction::South => [(-1, 1), (0, 1), (1, 1)],
            Direction::West => [(-1, -1), (-1, 0), (-1, 1)],
            Direction::East => [(1, -1), (1, 0), (1, 1)],
        };
        for direction in to_check {
            if map.contains(&(elf.0 + direction.0, elf.1 + direction.1)) {
                return false;
            }
        }
        return true;
    }

    fn shift(&self, elf: &Coordinate) -> Coordinate {
        let delta = match &self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        };
        (delta.0 + elf.0, delta.1 + elf.1)
    }
}

fn main() {
    let file = include_str!("../input.txt");
    let mut map: HashSet<Coordinate> = file
        .trim()
        .split('\n')
        .enumerate()
        .flat_map(|(y, chars)| {
            chars
                .bytes()
                .enumerate()
                .filter(|(_, char)| *char == b'#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .collect();

    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
    let mut round = 1;
    loop {
        // False means a single elf proposed moving to the coordinate, true otherwise.
        let mut all_elves_clear = true;
        let mut proposed_coordinates: HashMap<Coordinate, bool> = HashMap::new();
        let mut proposed_elf_position: HashMap<Coordinate, Coordinate> = HashMap::new();
        for elf in &map {
            let all_empty = directions
                .iter()
                .all(|direction| direction.is_clear(elf, &map));
            if all_empty {
                continue;
            }
            all_elves_clear = false;
            for direction in &directions {
                if direction.is_clear(&elf, &map) {
                    let proposed = direction.shift(elf);
                    let occupied = proposed_coordinates.contains_key(&proposed);
                    proposed_coordinates.insert(proposed.clone(), occupied);
                    proposed_elf_position.insert(*elf, proposed);
                    break;
                }
            }
        }

        if all_elves_clear {
            println!("All elves clear. Rounds: {round}");
            return;
        }

        let mut new_map: HashSet<Coordinate> = HashSet::new();
        for elf in &map {
            let proposed = proposed_elf_position.get(elf);
            if proposed.is_some() && !proposed_coordinates[proposed.unwrap()] {
                new_map.insert(*proposed.unwrap());
            } else {
                new_map.insert(*elf);
            }
        }
        map = new_map;
        directions.rotate_left(1);

        if round == ROUNDS {
            let mut min_x = i32::MAX;
            let mut max_x = i32::MIN;
            let mut min_y = i32::MAX;
            let mut max_y = i32::MIN;
            for elf in &map {
                if elf.0 < min_x {
                    min_x = elf.0;
                }
                if elf.0 > max_x {
                    max_x = elf.0;
                }
                if elf.1 < min_y {
                    min_y = elf.1;
                }
                if elf.1 > max_y {
                    max_y = elf.1;
                }
            }

            let mut count = 0;
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if !map.contains(&(x, y)) {
                        count += 1;
                    }
                }
            }
            println!("The result is {count}");
        }
        round += 1;
    }
}
