// Disclaimer:
// This is by far the worst code I've ever written.
// It works by folding each face into an actual 3d cube.
// There is A LOT that could be improved, but making this took WAY too long.
// Here be dragons.

use std::{collections::HashMap, fs};

use euclid::{default::Point3D, Angle, Rotation3D, UnknownUnit, Vector3D};

type Coordinate = (i64, i64);

const DIRECTIONS: &'static [(i64, i64)] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

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

#[allow(dead_code)]
struct BoundingBox {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

#[allow(dead_code)]
impl BoundingBox {
    fn from(map: &HashMap<Coordinate, bool>) -> BoundingBox {
        let mut bounding_box = BoundingBox {
            min_x: i64::MAX,
            max_x: 0,
            min_y: i64::MAX,
            max_y: 0,
        };
        for coordinate in map.keys() {
            bounding_box.min_x = bounding_box.min_x.min(coordinate.0);
            bounding_box.max_x = bounding_box.max_x.max(coordinate.0);
            bounding_box.min_y = bounding_box.min_y.min(coordinate.1);
            bounding_box.max_y = bounding_box.max_y.max(coordinate.1);
        }
        bounding_box
    }
}

#[allow(dead_code)]
fn write_map(map: &HashMap<Coordinate, Point3D<i64>>, width: i64, elf: Option<Point3D<i64>>) {
    unsafe {
        static mut NUM: usize = 0;
        let mut s = String::new();

        let colours = colour_map(map, width);
        let mut colour_indices: Vec<&i64> = colours.keys().collect();
        colour_indices.sort();
        for colour in colour_indices {
            for point in &colours[colour] {
                s += &format!("{}, {}, {}\n", point.x, point.y, point.z);
            }
            s += &format!("\n\n");
        }
        if let Some(elf) = elf {
            s += &format!("{}, {}, {}\n\n", elf.x, elf.y, elf.z);
        }
        fs::write(format!("points{}.txt", NUM), s).unwrap();
        NUM += 1;
    }
}

fn colour_map(
    map: &HashMap<Coordinate, Point3D<i64>>,
    width: i64,
) -> HashMap<i64, Vec<Point3D<i64>>> {
    let mut colours: HashMap<i64, Vec<Point3D<i64>>> = HashMap::new();
    let bounding_box = BoundingBox::from(&map.keys().map(|k| (k.clone(), false)).collect());
    for x in 0..=bounding_box.max_x {
        for y in 0..=bounding_box.max_y {
            if let Some(point) = map.get(&(x, y)) {
                let coord = x / width as i64 + (y / width as i64) * width as i64;
                if let Some(vec) = colours.get_mut(&coord) {
                    vec.push(point.clone());
                } else {
                    colours.insert(coord, vec![point.clone()]);
                }
            }
        }
    }
    colours
}

fn top_right_corner(flat_map: &HashMap<Coordinate, bool>) -> Coordinate {
    *flat_map
        .keys()
        .filter(|a| a.1 == 0)
        .reduce(|a, b| if a.0 < b.0 { a } else { b })
        .unwrap()
}

fn fold_cube(
    flat_map: &HashMap<Coordinate, bool>,
    map: &mut HashMap<Coordinate, Point3D<i64>>,
    width: i64,
) -> (
    HashMap<Point3D<i64>, Vector3D<f64, UnknownUnit>>,
    HashMap<Coordinate, Rotation3D<f64, UnknownUnit, UnknownUnit>>,
) {
    println!("Folding cube");
    let corner_a = &top_right_corner(flat_map);
    println!("The top right corner is {:?}", corner_a);
    let mut normals: HashMap<Point3D<i64>, Vector3D<f64, UnknownUnit>> = HashMap::new();
    let mut rotations: HashMap<Coordinate, Rotation3D<f64, UnknownUnit, UnknownUnit>> =
        HashMap::new();
    for point in map.iter() {
        rotations.insert(point.0.clone(), Rotation3D::identity());
        normals.insert(point.1.clone(), Vector3D::from((0f64, 0f64, 1f64)));
    }

    fold_cube_from(
        corner_a,
        &(-1, -1),
        flat_map,
        map,
        width,
        &mut normals,
        &mut rotations,
    );
    (normals, rotations)
}

fn fold_cube_from(
    destination: &Coordinate,
    source: &Coordinate,
    flat_map: &HashMap<Coordinate, bool>,
    map: &mut HashMap<Coordinate, Point3D<i64>>,
    width: i64,
    normals: &mut HashMap<Point3D<i64>, Vector3D<f64, UnknownUnit>>,
    rotations: &mut HashMap<Coordinate, Rotation3D<f64, UnknownUnit, UnknownUnit>>,
) {
    for direction in DIRECTIONS {
        let new_destination = (
            destination.0 + direction.0 * width,
            destination.1 + direction.1 * width,
        );
        if new_destination == *source || !flat_map.contains_key(&new_destination) {
            continue;
        }
        let attached = find_attached(&flat_map, &new_destination, &destination, width);
        let axis = match direction {
            // Left:
            (-1, 0) => [
                (destination.0, destination.1),
                (destination.0, destination.1 + width - 1),
            ],

            // Right:
            (1, 0) => [
                (destination.0 + width - 1, destination.1 + width - 1),
                (destination.0 + width - 1, destination.1),
            ],

            // Up:
            (0, -1) => [
                (destination.0, destination.1),
                (destination.0 + width - 1, destination.1),
            ],

            // Down:
            (0, 1) => [
                (destination.0, destination.1 + width - 1),
                (destination.0 + width - 1, destination.1 + width - 1),
            ],
            _ => unreachable!(),
        };
        println!("Folding {:?}", direction);
        println!("Folding {} tiles along {:?}", attached.len(), axis);

        let rotation_translation = map[&axis[0]].to_vector();
        let close_distance_transation = map[&destination] - map[&new_destination];
        let close_distance_transation: Vector3D<f64, UnknownUnit> = Vector3D::from((
            close_distance_transation.x as f64,
            close_distance_transation.y as f64,
            close_distance_transation.z as f64,
        ));
        let close_distance_transation = close_distance_transation.normalize();
        let close_distance_transation: Vector3D<i64, UnknownUnit> = Vector3D::from((
            close_distance_transation.x.round() as i64,
            close_distance_transation.y.round() as i64,
            close_distance_transation.z.round() as i64,
        ));

        let axis = map[&axis[1]] - map[&axis[0]];
        let axis = Vector3D::from((axis.x as f64, axis.y as f64, axis.z as f64));

        let rotation: Rotation3D<f64, UnknownUnit, UnknownUnit> =
            Rotation3D::around_axis(axis, Angle::degrees(90f64));
        println!("Translation vector : {:?}", rotation_translation);
        println!("Rotation axis: {:?}", axis);

        let normal = normals[&map[&new_destination]];
        let normal = rotation.transform_vector3d(normal);
        println!(
            "Normal: {}, {}, {}",
            normal.x.round(),
            normal.y.round(),
            normal.z.round()
        );

        for tile in attached {
            let point = map[&tile];
            let point = point - rotation_translation;
            let point = rotate_point(&point, &rotation);
            let point = point + rotation_translation;
            let point = point - close_distance_transation;
            map.insert(tile.clone(), point);
            normals.insert(point, normal);
            rotations.insert(tile, rotations[&tile].then(&rotation));
        }
        //write_map(map, width, None);
        fold_cube_from(
            &new_destination,
            destination,
            flat_map,
            map,
            width,
            normals,
            rotations,
        );
    }
}

fn rotate_point(
    point: &Point3D<i64>,
    transform: &Rotation3D<f64, UnknownUnit, UnknownUnit>,
) -> Point3D<i64> {
    let point = Point3D::from((point.x as f64, point.y as f64, point.z as f64));
    let point = transform.transform_point3d(point);
    let point = Point3D::from((
        point.x.round() as i64,
        point.y.round() as i64,
        point.z.round() as i64,
    ));
    point
}

fn rotate_vector(
    point: &Vector3D<i64, UnknownUnit>,
    transform: &Rotation3D<f64, UnknownUnit, UnknownUnit>,
) -> Vector3D<i64, UnknownUnit> {
    let point = Vector3D::from((point.x as f64, point.y as f64, point.z as f64));
    let point = transform.transform_vector3d(point);
    let point = Vector3D::from((
        point.x.round() as i64,
        point.y.round() as i64,
        point.z.round() as i64,
    ));
    point
}

fn find_attached(
    flat_map: &HashMap<Coordinate, bool>,
    destination: &Coordinate,
    source: &Coordinate,
    width: i64,
) -> Vec<Coordinate> {
    if !flat_map.contains_key(destination) {
        return vec![];
    }
    let mut attached: Vec<Coordinate> = Vec::new();
    for x in destination.0..destination.0 + width {
        for y in destination.1..destination.1 + width {
            assert!(flat_map.contains_key(&(x, y)));
            attached.push((x, y));
        }
    }
    for direction in DIRECTIONS {
        let new_destination = (
            destination.0 + direction.0 * width,
            destination.1 + direction.1 * width,
        );
        if new_destination == *source {
            continue;
        }
        attached.append(&mut find_attached(
            flat_map,
            &new_destination,
            destination,
            width,
        ));
    }
    attached
}

fn walk(
    flat_map: &HashMap<Coordinate, bool>,
    map: &HashMap<Coordinate, Point3D<i64>>,
    normals: &HashMap<Point3D<i64>, Vector3D<f64, UnknownUnit>>,
    rotations: &HashMap<Coordinate, Rotation3D<f64, UnknownUnit, UnknownUnit>>,
    path: Path,
    #[allow(unused_variables)]
    width: i64,
) -> i64 {
    let position = top_right_corner(flat_map);
    let mut position = map[&position];

    let map_3d: HashMap<Point3D<i64>, bool> = map
        .iter()
        .map(|(coordinate, point3d)| (*point3d, flat_map[coordinate]))
        .collect();
    let mut direction: Vector3D<i64, UnknownUnit> = Vector3D::from((1, 0, 0));

    let total = path.instructions.len();
    for (i, instruction) in path.instructions.iter().enumerate() {
        if i % 100 == 0 {
            println!("Processing {} of {}", i + 1, total);
        }

        let mut normal = normals[&position];
        match instruction {
            Instruction::Left => {
                let rotation: Rotation3D<f64, UnknownUnit, UnknownUnit> =
                    Rotation3D::around_axis(normal, Angle::degrees(-90f64));
                direction = rotate_vector(&direction, &rotation);
            }
            Instruction::Right => {
                let rotation: Rotation3D<f64, UnknownUnit, UnknownUnit> =
                    Rotation3D::around_axis(normal, Angle::degrees(90f64));
                direction = rotate_vector(&direction, &rotation);
            }
            Instruction::Forward(steps) => {
                for _ in 0..*steps {
                    let mut next = position + direction;
                    if let Some(obstacle) = map_3d.get(&next) {
                        if *obstacle {
                            break;
                        }
                        position = next;
                        //write_map(map, width, Some(position));
                    } else {
                        // Wrap around.
                        let maybe_direction =
                            Vector3D::from((normal.x as i64, normal.y as i64, normal.z as i64));
                        next = next + maybe_direction;
                        if map_3d[&next] {
                            break;
                        }
                        direction = maybe_direction;
                        position = next;
                        //write_map(map, width, Some(position));
                        normal = normals[&position];
                    }
                }
            }
        }
    }

    println!("Finished");

    let final_position = *map.iter().find(|(_, point)| **point == position).unwrap().0;
    println!("Final position in 2D: {:?}", final_position);
    println!("Final position in 3D: {:?}", position);

    let column = final_position.0 + 1;
    let row = final_position.1 + 1;
    println!("Column: {}", column);
    println!("Row: {}", row);

    println!("Direction in 3D: {:?}", direction);

    let rotation = rotations[&final_position];
    let direction = DIRECTIONS
        .iter()
        .map(|direction| rotate_vector(&Vector3D::from((direction.0, direction.1, 0)), &rotation))
        .enumerate()
        .inspect(|(_, dir)| println!("Rotated directions: {:?}", dir))
        .find(|(_, a)| *a == direction)
        .unwrap()
        .0 as i64;

    println!("Direction value in 2D: {}", direction);

    1000 * row + 4 * column + direction
}

fn main() {
    let file = include_str!("../input.txt");
    let mut parts = file.split("\n\n");

    // Parse the map.
    let map = parts.next().unwrap();
    let flat_map: HashMap<Coordinate, bool> = map
        .split('\n')
        .enumerate()
        .flat_map(|(y, chars)| {
            chars
                .bytes()
                .enumerate()
                .filter(|(_, char)| !char.is_ascii_whitespace())
                .map(move |(x, char)| ((x as i64, y as i64), char == b'#'))
        })
        .collect();

    // Find the width of each face.
    let width = ((flat_map.len() / 6) as f64).sqrt() as i64;
    println!("The tile width is {width}");

    // Convert the flat map into a 3D map.
    let mut map: HashMap<Coordinate, Point3D<i64>> = flat_map
        .keys()
        .map(|point2d| {
            (
                point2d.clone(),
                Point3D::from((point2d.0 as i64, point2d.1 as i64, 0)),
            )
        })
        .collect();

    // Fold the cube.
    //write_map(&map, width, None);
    let (normals, rotations) = fold_cube(&flat_map, &mut map, width);

    // Parse the path.
    let path = Path::from(parts.next().unwrap());

    let result = walk(&flat_map, &map, &normals, &rotations, path, width);
    println!("The result is {:?}", result);
}
