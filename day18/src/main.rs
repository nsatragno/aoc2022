use std::{
    collections::{HashMap, HashSet},
    fs,
    str::Split,
};

type Coordinate = (i64, i64, i64);

fn next_digit(line: &mut Split<char>) -> i64 {
    line.next().unwrap().parse().unwrap()
}

fn add(coordinates: &Coordinate, delta: &Coordinate) -> Coordinate {
    (
        coordinates.0 + delta.0,
        coordinates.1 + delta.1,
        coordinates.2 + delta.2,
    )
}

fn explore_exterior(
    cubes: &HashMap<Coordinate, i64>,
    visited: &mut HashSet<Coordinate>,
    bottom_corner: &Coordinate,
    top_corner: &Coordinate,
    cube: Coordinate,
) {
    if cube.0 < bottom_corner.0
        || cube.1 < bottom_corner.1
        || cube.2 < bottom_corner.2
        || cube.0 > top_corner.0
        || cube.1 > top_corner.1
        || cube.2 > top_corner.2
    {
        return;
    }
    if visited.contains(&cube) {
        return;
    }
    visited.insert(cube);

    let sides = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    for side in sides {
        let next = add(&cube, &side);
        if !cubes.contains_key(&next) {
            explore_exterior(cubes, visited, bottom_corner, top_corner, next);
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut cubes: HashMap<Coordinate, i64> = HashMap::new();
    for line in file.trim().split('\n') {
        let mut line = line.trim().split(',');
        let coordinates: Coordinate = (
            next_digit(&mut line),
            next_digit(&mut line),
            next_digit(&mut line),
        );
        let sides = [
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ];
        let mut number_sides = 6;
        for side in sides {
            let side = add(&coordinates, &side);
            if let Some(other_cube) = cubes.get_mut(&side) {
                *other_cube -= 1;
                number_sides -= 1;
            }
        }
        cubes.insert(coordinates, number_sides);
    }

    let result: i64 = cubes.values().sum();
    println!("The result is {result}");

    // Find a cube that envelops the structure.
    let bottom_corner = cubes.keys().fold((i64::MAX, i64::MAX, i64::MAX), |a, b| {
        (a.0.min(b.0), a.1.min(b.1), a.2.min(b.2))
    });
    let bottom_corner = add(&bottom_corner, &(-1, -1, -1));
    let top_corner = cubes
        .keys()
        .fold((0, 0, 0), |a, b| (a.0.max(b.0), a.1.max(b.1), a.2.max(b.2)));
    let top_corner = add(&top_corner, &(1, 1, 1));

    let mut visited: HashSet<Coordinate> = HashSet::new();
    explore_exterior(
        &cubes,
        &mut visited,
        &bottom_corner,
        &top_corner,
        bottom_corner,
    );

    let result: usize = cubes
        .keys()
        .map(|cube| {
            [
                (1, 0, 0),
                (-1, 0, 0),
                (0, 1, 0),
                (0, -1, 0),
                (0, 0, 1),
                (0, 0, -1),
            ]
            .iter()
            .filter(|side| visited.contains(&add(cube, side)))
            .count()
        })
        .sum();
    println!("The result is {result}");
}
