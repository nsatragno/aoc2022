use std::{collections::HashSet, fs};

struct Command<'a> {
    direction: &'a str,
    steps: u32,
}

fn pull(head: i32, tail: &mut i32) {
    *tail += (head - *tail).signum();
}

fn move_head(direction: &str, head: &mut (i32, i32)) {
    match direction {
        "R" => head.0 += 1,
        "L" => head.0 -= 1,
        "U" => head.1 += 1,
        "D" => head.1 -= 1,
        _ => panic!("Unrecognized direction"),
    }
}

fn move_tail(head: &(i32, i32), tail: &mut (i32, i32)) {
    if (tail.0 - head.0).abs() > 1 {
        // X displacement.
        pull(head.0, &mut tail.0);
        if (tail.1 - head.1).abs() > 0 {
            // Diagonal displacement.
            pull(head.1, &mut tail.1);
        }
    } else if (tail.1 - head.1).abs() > 1 {
        // Y displacement.
        pull(head.1, &mut tail.1);
        if (tail.0 - head.0).abs() > 0 {
            // Diagonal displacement.
            pull(head.0, &mut tail.0);
        }
    }
}

fn parse(line: &str) -> Command {
    let mut parts = line.trim().split_whitespace();
    Command {
        direction: parts.next().unwrap(),
        steps: parts.next().unwrap().parse().unwrap(),
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut positions = HashSet::new();
    positions.insert(tail);
    for line in file.trim().split('\n') {
        let command = parse(line);
        for _ in 0..command.steps {
            move_head(command.direction, &mut head);
            move_tail(&head, &mut tail);
            positions.insert(tail);
        }
    }

    println!("Total positions: {}", positions.len());

    // Second part:
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    positions.insert(*rope.last().unwrap());
    for line in file.trim().split('\n') {
        let command = parse(line);
        for _ in 0..command.steps {
            move_head(command.direction, &mut rope[0]);
            for i in 0..rope.len() - 1 {
                let head = rope[i];
                move_tail(&head, &mut rope[i + 1]);
            }
            positions.insert(*rope.last().unwrap());
        }
    }

    println!("Total positions: {}", positions.len());
}
