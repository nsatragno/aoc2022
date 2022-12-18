use std::{fs};

type Field = Vec<u8>;

const BLOCKS: usize = 10;
const BLOCKS_PART_2: usize = 2022;
// const BLOCKS_PART_2: usize = 1_000_000_000_000;

struct Piece {
    shape: Vec<u8>,
    position: usize,
}

impl Piece {
    fn from(shape: Vec<u8>, ceiling: usize) -> Piece {
        Piece {
            shape,
            position: 3 + ceiling,
        }
    }

    fn maybe_move_left(&mut self, field: &mut Field) -> bool {
        for index in 0..self.shape.len() {
            let line = self.shape[index];
            if line & 0b1000000 != 0 {
                return false;
            }
            let line = line << 1;
            if (field[index + self.position] & line) != 0 {
                return false;
            }
        }
        for index in 0..self.shape.len() {
            self.shape[index] <<= 1;
        }
        true
    }

    fn maybe_move_right(&mut self, field: &mut Field) -> bool {
        for index in 0..self.shape.len() {
            let line = self.shape[index];
            if line & 0b0000001 != 0 {
                return false;
            }
            let line = line >> 1;
            if (field[index + self.position] & line) != 0 {
                return false;
            }
        }
        for index in 0..self.shape.len() {
            self.shape[index] >>= 1;
        }
        true
    }

    fn maybe_move_down(&mut self, field: &mut Field) -> bool {
        if self.position <= 0 {
            return false;
        }
        for index in 0..self.shape.len() {
            let line = self.shape[index];
            if (field[index + self.position - 1] & line) != 0 {
                return false;
            }
        }
        self.position -= 1;
        true
    }

    fn paint(&self, field: &mut Field) {
        for index in 0..self.shape.len() {
            let line = self.shape[index];
            field[index + self.position] |= line;
        }
    }
}

fn solve(iterations: usize) {
    let line: Vec<u8> = vec![0b0011110];
    let cross: Vec<u8> = vec![0b0001000, 0b0011100, 0b0001000];
    let l: Vec<u8> = vec![0b0011100,0b0000100, 0b0000100];
    let stick: Vec<u8> = vec![0b0010000,0b0010000,0b0010000,0b0010000];
    let square: Vec<u8> = vec![0b0011000, 0b0011000];
    let shapes = vec![line, cross, l, stick, square];
    let mut shapes = shapes.iter().cycle();

    let input = fs::read_to_string("input.txt").unwrap();
    let directions: Vec<i32> = input
        .trim()
        .bytes()
        .map(|character| match character {
            b'>' => 1,
            b'<' => -1,
            _ => panic!("Unknown character {}", character),
        }).collect();

    let mut field: Field = Vec::new();
    let mut ceiling = 0;
    let mut direction_index = 0;
    for iteration in 0..iterations {
        if iteration % 1024 == 0 {
            let done: f32 = iteration as f32 / iterations as f32;
            println!("%{:02.02}", done * 100 as f32)
        }
        let shape = shapes.next().unwrap();
        let mut piece = Piece::from(shape.clone(), ceiling);
        if piece.position + 5 > field.len() {
            field.resize(piece.position + 5, 0);
        }
        loop {
            let direction = directions[direction_index];
            if direction == -1 {
                piece.maybe_move_left(&mut field);
            } else {
                piece.maybe_move_right(&mut field);
            }
            direction_index += 1;
            if direction_index >= directions.len() {
                direction_index = 0;
            }
            if !piece.maybe_move_down(&mut field) {
                break;
            }
        }
        piece.paint(&mut field);
        for (index, line) in field.iter().enumerate().rev() {
            if *line != 0 {
                ceiling = index + 1;
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