use std::{collections::HashMap, fs, time::Instant};

type Field = Vec<u8>;

const BLOCKS: usize = 2022;
//const BLOCKS_PART_2: usize = 100_000_000000;
const BLOCKS_PART_2: usize = 1_000_000_000_000;

#[derive(PartialEq, Eq, Hash, Clone)]
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

    fn can_move_left(&mut self, field: &Field) -> bool {
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
        true
    }

    fn maybe_move_left(&mut self, field: &Field) -> bool {
        if !self.can_move_left(field) {
            return false;
        }
        for index in 0..self.shape.len() {
            self.shape[index] <<= 1;
        }
        true
    }

    fn can_move_right(&mut self, field: &Field) -> bool {
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
        true
    }

    fn maybe_move_right(&mut self, field: &Field) -> bool {
        if !self.can_move_right(field) {
            return false;
        }
        for index in 0..self.shape.len() {
            self.shape[index] >>= 1;
        }
        true
    }

    fn maybe_move_down(&mut self, field: &Field) -> bool {
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

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    reduced_field: Vec<u8>,
    direction_index: usize,
    shape_index: usize,
}

/*fn try_depth(field: &Field, mut piece: Piece) -> usize {
    let mut depth = piece.position;
    if !piece.maybe_move_down(field) {
        return depth;
    }
    let mut leftie = piece.clone();
    while leftie.maybe_move_left(field) {
        depth = depth.min(try_depth(field, leftie.clone()));
    }
    let mut rightie = piece;
    while rightie.maybe_move_right(field) {
        depth = depth.min(try_depth(field, rightie.clone()));
    }
    return depth;
}*/

fn reduce_field(field: &Field, _ceiling: usize) -> Vec<u8> {
    // This number seems to work.
    // TODO: find a better way to calculate this to guarantee a correct response.
    let depth = 20.min(field.len());
    field[field.len() - depth..].iter().map(|s| *s).collect()
}

fn solve(iterations: usize) {
    let line_piece: Vec<u8> = vec![0b0011110];
    let cross_piece: Vec<u8> = vec![0b0001000, 0b0011100, 0b0001000];
    let l_piece: Vec<u8> = vec![0b0011100, 0b0000100, 0b0000100];
    let stick_piece: Vec<u8> = vec![0b0010000, 0b0010000, 0b0010000, 0b0010000];
    let square_piece: Vec<u8> = vec![0b0011000, 0b0011000];
    let shapes: [Vec<u8>; 5] = [line_piece, cross_piece, l_piece, stick_piece, square_piece];

    let input = fs::read_to_string("input.txt").unwrap();
    let directions: Vec<i32> = input
        .trim()
        .bytes()
        .map(|character| match character {
            b'>' => 1,
            b'<' => -1,
            _ => panic!("Unknown character {}", character),
        })
        .collect();

    let mut field: Field = Vec::new();
    field.reserve(iterations * 3);
    let mut ceiling = 0;
    let mut direction_index = 0;
    let mut shape_index = 0;
    let mut states: HashMap<State, usize> = HashMap::new();
    let mut ceilings: Vec<usize> = Vec::new();
    for iteration in 0..iterations {
        let shape = shapes[shape_index].clone();
        shape_index += 1;
        if shape_index >= shapes.len() {
            shape_index = 0;
        }
        let mut piece = Piece::from(shape.clone(), ceiling);
        if piece.position + 5 > field.len() {
            field.resize(piece.position + 5, 0);
        }

        let reduced_field = reduce_field(&field, ceiling);
        let state = State {
            reduced_field,
            direction_index,
            shape_index,
        };
        if let Some(previous_iteration) = states.insert(state.clone(), iteration) {
            println!("SUCCESS!!!!");
            println!("Iteration: {iteration}");
            let remaining_iterations = iterations - iteration;
            let loop_iterations = iteration - previous_iteration;
            let loop_ceiling = ceiling - ceilings[previous_iteration];
            let remaining_ceiling = ceilings
                [previous_iteration + remaining_iterations % loop_iterations]
                - ceilings[previous_iteration];
            let loops = remaining_iterations / loop_iterations;

            let result = ceiling + loops * loop_ceiling + remaining_ceiling;

            println!("The result is {}", result);
            return;
        }
        ceilings.push(ceiling);

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
    let then = Instant::now();
    solve(BLOCKS);
    solve(BLOCKS_PART_2);
    println!("This took {:?}", Instant::now() - then);
}
