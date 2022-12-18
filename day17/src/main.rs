use std::fs;

type Coordinate = (i32, i32);
type Field = Vec<[bool; 7]>;

const BLOCKS: usize = 2022;
// const BLOCKS_PART_2: usize = 1_000_000_000_000;
const BLOCKS_PART_2: usize = 2022;

struct Piece<'a> {
    shape: &'a Vec<Coordinate>,
    position: Coordinate,
}

impl<'a> Piece<'a> {
    fn from(shape: &'a Vec<Coordinate>, ceiling: i32) -> Piece<'a> {
        Piece {
            shape,
            position: (2, 3 + ceiling),
        }
    }

    fn maybe_move(&mut self, direction: Coordinate, field: &mut Field) -> bool {
        let can_move = !self
            .shape
            .iter()
            .map(|pixel| {
                (
                    self.position.0 + direction.0 + pixel.0,
                    self.position.1 + direction.1 + pixel.1,
                )
            })
            .any(|coordinate| {
                coordinate.0 < 0
                    || coordinate.0 > 6
                    || coordinate.1 < 0
                    || field[coordinate.1 as usize][coordinate.0 as usize]
            });
        if can_move {
            self.position.0 += direction.0;
            self.position.1 += direction.1;
        }
        can_move
    }

    fn paint(&self, field: &mut Field) {
        for pixel in self.shape {
            let position = (pixel.0 + self.position.0, pixel.1 + self.position.1);
            assert!(!field[position.1 as usize][position.0 as usize]);
            field[position.1 as usize][position.0 as usize] = true;
        }
    }

    fn top(&self) -> usize {
        self.shape
            .iter()
            .map(|pixel| (pixel.1 + self.position.1) as usize)
            .max()
            .unwrap()
    }
}

fn solve(iterations: usize) {
    let line: Vec<Coordinate> = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let cross: Vec<Coordinate> = vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
    let l: Vec<Coordinate> = vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];
    let stick: Vec<Coordinate> = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let square: Vec<Coordinate> = vec![(0, 0), (0, 1), (1, 0), (1, 1)];
    let shapes = vec![line, cross, l, stick, square];
    let mut shapes = shapes.iter().cycle();

    let input = fs::read_to_string("input.txt").unwrap();
    let mut directions = input
        .trim()
        .bytes()
        .map(|character| match character {
            b'>' => 1,
            b'<' => -1,
            _ => panic!("Unknown character {}", character),
        })
        .cycle();

    let mut field: Field = Vec::new();
    let mut ceiling = 0;
    for iteration in 0..iterations {
        if iteration % 1024 == 0 {
            let done: f32 = iteration as f32 / iterations as f32;
            println!("%{:02.02}", done * 100 as f32)
        }
        let shape = shapes.next().unwrap();
        let mut piece = Piece::from(shape, ceiling);
        if piece.top() + 1 > field.len() {
            field.resize(piece.top() + 1, [false; 7]);
        }
        loop {
            let direction = directions.next().unwrap();
            piece.maybe_move((direction, 0), &mut field);
            if !piece.maybe_move((0, -1), &mut field) {
                break;
            }
        }
        piece.paint(&mut field);
        for (index, line) in field.iter().enumerate().rev() {
            if line.contains(&true) {
                ceiling = index as i32 + 1;
                break;
            }
        }
    }
    println!("The result is {ceiling}");
}

fn main() {
    solve(BLOCKS);
    solve(BLOCKS_PART_2);
}