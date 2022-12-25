use std::{collections::HashMap, fs, ops::Bound};

use euclid::{default::Point3D, Angle, Rotation3D, UnknownUnit, Vector3D};

type Coordinate = (i64, i64);

const DIRECTIONS: &'static [(i64, i64)] = &[(-1, 0), (1, 0), (0, 1), (0, -1)];

struct BoundingBox {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

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

fn write_map(map: &HashMap<Coordinate, Point3D<i64>>, width: i64) {
    unsafe {
        static mut NUM: usize = 0;
        let mut s = String::new();

        let colours = colour_map(map, width);
        for colour in colours.values() {
            for point in colour {
                s += &format!("{}, {}, {}\n", point.x, point.y, point.z);
            }
            s += &format!("\n\n");
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

fn fold_cube(
    flat_map: &HashMap<Coordinate, bool>,
    map: &mut HashMap<Coordinate, Point3D<i64>>,
    width: i64,
) {
    println!("Folding cube");

    // Find the top-right corner.
    let corner_a = flat_map
        .keys()
        .filter(|a| a.1 == 0)
        .reduce(|a, b| if a.0 < b.0 { a } else { b })
        .unwrap();

    println!("The top right corner is {:?}", corner_a);

    fold_cube_from(corner_a, &(-1, -1), flat_map, map, width);
}

fn fold_cube_from(
    destination: &Coordinate,
    source: &Coordinate,
    flat_map: &HashMap<Coordinate, bool>,
    map: &mut HashMap<Coordinate, Point3D<i64>>,
    width: i64,
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
        for tile in attached {
            let point = map[&tile];
            let point = point - rotation_translation;
            let point = Point3D::from((point.x as f64, point.y as f64, point.z as f64));
            let point = rotation.transform_point3d(point);
            let point = Point3D::from((
                point.x.round() as i64,
                point.y.round() as i64,
                point.z.round() as i64,
            ));
            let point = point + rotation_translation;
            let point = point - close_distance_transation;
            map.insert(tile.clone(), point);
        }
        write_map(map, width);
        fold_cube_from(&new_destination, destination, flat_map, map, width);
    }
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

    write_map(&map, width);
    fold_cube(&flat_map, &mut map, width);
}
