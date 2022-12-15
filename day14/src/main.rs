use std::fs;

type Coordinate = (usize, usize);

const SOURCE: usize = 500;

fn parse_coordinate(string: &str) -> Coordinate {
    let mut parts = string.split(',');
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap(),
    )
}

fn iterate_range(left: usize, right: usize) -> std::ops::RangeInclusive<usize> {
    if left < right {
        left..=right
    } else {
        right..=left
    }
}

fn fill(source: usize, mut cave: Vec<Vec<char>>) -> u32 {
    let mut grains = 0;
    'next_grain: loop {
        let mut grain = (source, 0);
        if cave[grain.0][grain.1] != '.' {
            println!("Completely filled");
            return grains;
        }
        loop {
            if grain.1 + 1 >= cave[0].len() {
                println!("Grain falling through the void");
                return grains;
            }
            if cave[grain.0][grain.1 + 1] == '.' {
                grain.1 += 1;
            } else if cave[grain.0 - 1][grain.1 + 1] == '.' {
                grain.0 -= 1;
                grain.1 += 1;
            } else if cave[grain.0 + 1][grain.1 + 1] == '.' {
                grain.0 += 1;
                grain.1 += 1;
            } else {
                cave[grain.0][grain.1] = 'o';
                grains += 1;
                continue 'next_grain;
            }
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let lines: Vec<Vec<Coordinate>> = file
        .trim()
        .split('\n')
        .map(|line| line.trim().split(" -> ").map(parse_coordinate).collect())
        .collect();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in lines.iter().flatten() {
        if line.0 > max_x {
            max_x = line.0;
        }
        if line.1 > max_y {
            max_y = line.1;
        }
    }
    let mut cave = vec![vec!['.'; max_y + 1]; max_x + 1];
    assert!(max_x >= SOURCE);

    for line in lines {
        for index in 1..line.len() {
            let first = line[index - 1];
            let second = line[index];
            if first.0 == second.0 {
                // Vertical line.
                for y in iterate_range(first.1, second.1) {
                    cave[first.0][y] = '#';
                }
            } else {
                // Horizontal line.
                for x in iterate_range(first.0, second.0) {
                    cave[x][first.1] = '#';
                }
            }
        }
    }

    println!("First part");
    println!("Units of sand: {}", fill(SOURCE, cave.clone()));

    // Add the extra space on the sides.
    let min_space_x = max_y + 2;
    let mut source = SOURCE;
    if min_space_x > source {
        // Left side:
        let additional_left_space = min_space_x - SOURCE + 1;
        source += additional_left_space;
        cave.resize(cave.len() + additional_left_space, vec!['.'; cave[0].len()]);
        cave.rotate_right(additional_left_space);
    }
    if min_space_x > (cave.len() - source) {
        // Right side:
        let additional_right_space = min_space_x - (cave.len() - source) + 1;
        cave.resize(
            cave.len() + additional_right_space,
            vec!['.'; cave[0].len()],
        );
    }

    // Add the floor.
    for x in 0..cave.len() {
        cave[x].push('.');
        cave[x].push('#');
    }

    println!("Second part");
    println!("Units of sand: {}", fill(source, cave));
}
