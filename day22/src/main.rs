use std::{fmt::Display, fs};

#[derive(Debug)]
struct Row {
    shift: i32,
    path: Vec<bool>,
}

impl Row {
    fn from(string: &str) -> Row {
        let mut chars = string.bytes().peekable();
        let mut shift = 0;
        while chars.peek().unwrap().is_ascii_whitespace() {
            chars.next();
            shift += 1;
        }
        let path = chars
            .filter(|char| !char.is_ascii_whitespace())
            .map(|char| {
                assert!(char == b'#' || char == b'.');
                char == b'#'
            })
            .collect();
        Row { shift, path }
    }

    fn len(&self) -> i32 {
        self.path.len() as i32 + self.shift
    }

    fn is_occupied(&self, x: i32) -> bool {
        assert!(x >= self.shift && x < self.len());
        self.path[(x - self.shift) as usize]
    }
}

#[derive(PartialEq)]
enum Mode {
    Flat,
    Cube,
}

#[derive(Debug)]
struct Map {
    rows: Vec<Row>,
}

impl Map {
    fn from(string: &str) -> Map {
        Map {
            rows: string.split('\n').map(Row::from).collect(),
        }
    }

    fn starting_position(&self) -> Position {
        let leftmost_tile = self.rows[0]
            .path
            .iter()
            .enumerate()
            .find(|(_, &occupied)| !occupied)
            .expect("Could not find open tile")
            .0;
        Position {
            x: leftmost_tile as i32 + self.rows[0].shift,
            y: 0,
            facing: Direction::Right,
        }
    }

    fn walk(&self, path: &Path, mut position: Position, mode: Mode) -> Position {
        for instruction in &path.instructions {
            match instruction {
                Instruction::Left => position.turn_left(),
                Instruction::Right => position.turn_right(),
                Instruction::Forward(steps) => {
                    for _ in 0..*steps {
                        let mut next = position.peek();
                        if mode == Mode::Flat {
                            next = self.maybe_wrap(next);
                        } else {
                            next = self.maybe_wrap_cube(next);
                        }
                        if self.is_occupied(&next) {
                            break;
                        }
                        position = next;
                    }
                }
            }
        }
        position
    }

    fn is_occupied(&self, position: &Position) -> bool {
        self.rows[position.y as usize].is_occupied(position.x)
    }

    fn maybe_wrap(&self, mut position: Position) -> Position {
        if position.y < 0
            || position.y >= self.rows.len() as i32
            || position.x < self.rows[position.y as usize].shift
            || position.x >= self.rows[position.y as usize].len()
        {
            // First, wrap the cursor.
            match position.facing {
                Direction::Up => position.y = self.rows.len() as i32 - 1,
                Direction::Down => position.y = 0,
                Direction::Left => position.x = self.rows[position.y as usize].len() as i32 - 1,
                Direction::Right => position.x = self.rows[position.y as usize].shift,
            }
            // Then, walk until we find land.
            while position.x < self.rows[position.y as usize].shift
                || position.x >= self.rows[position.y as usize].len()
            {
                position.step();
            }
        }
        position
    }

    fn maybe_wrap_cube(&self, mut position: Position) -> Position {
        todo!();
        position
    }
}

#[derive(Debug)]
enum Instruction {
    Forward(i32),
    Left,
    Right,
}

impl Instruction {
    fn from(string: &str) -> Instruction {
        match string {
            "L" => Instruction::Left,
            "R" => Instruction::Right,
            number => Instruction::Forward(number.parse().unwrap()),
        }
    }
}

#[derive(Debug)]
struct Path {
    instructions: Vec<Instruction>,
}

impl Path {
    fn from(string: &str) -> Path {
        let mut instructions = Vec::new();
        let mut string = string.trim().chars();
        let mut number = String::new();
        while let Some(next) = string.next() {
            if next.is_digit(10) {
                number.push(next);
            } else {
                if !number.is_empty() {
                    instructions.push(Instruction::from(&number));
                    number.clear();
                }
                instructions.push(Instruction::from(&next.to_string()));
            }
        }
        if !number.is_empty() {
            instructions.push(Instruction::from(&number));
        }
        Path { instructions }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn value(&self) -> i32 {
        match self {
            Direction::Up => 3,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 0,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => f.write_str("Up"),
            Direction::Down => f.write_str("Down"),
            Direction::Left => f.write_str("Left"),
            Direction::Right => f.write_str("Right"),
        }
        .unwrap();
        f.write_fmt(format_args!(" ({})", self.value()))
    }
}

#[derive(Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
    facing: Direction,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Row: {}, Column: {}, Direction: {}",
            self.y + 1,
            self.x + 1,
            self.facing
        ))
    }
}

impl Position {
    fn turn_left(&mut self) {
        self.facing = match self.facing {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn peek(&self) -> Position {
        let mut x = self.x;
        let mut y = self.y;
        match self.facing {
            Direction::Up => y -= 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        };
        Position {
            x,
            y,
            facing: self.facing,
        }
    }

    fn step(&mut self) {
        *self = self.peek();
    }

    fn to_result(&self) -> i32 {
        (self.y + 1) * 1000 + (self.x + 1) * 4 + self.facing.value()
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut parts = file.split("\n\n");
    let map = Map::from(parts.next().unwrap());
    let path = Path::from(parts.next().unwrap());

    let starting_position = map.starting_position();
    println!("Starting position: {}", starting_position);

    println!("Part 1");
    let finish_position = map.walk(&path, starting_position.clone(), Mode::Flat);
    println!("Finish position: {}", &finish_position);

    let result = finish_position.to_result();
    println!("         Result: {}", result);

    println!("Part 2");
    let finish_position = map.walk(&path, starting_position, Mode::Cube);
    println!("Finish position: {}", &finish_position);

    let result = finish_position.to_result();
    println!("         Result: {}", result);
}
