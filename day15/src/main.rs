use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::RangeInclusive,
    str::Chars,
};

type Coordinate = (i64, i64);

const TARGET_Y: i64 = 2_000_000;
const SEARCH_RANGE: i64 = 4_000_000;

struct Sensor {
    position: Coordinate,
    beacon: Coordinate,
}

impl Sensor {
    fn from(string: &str) -> Sensor {
        let mut string = string.chars();
        fn find_number(string: &mut Chars) -> i64 {
            let mut number = String::new();
            while let Some(char) = string.next() {
                if char.is_numeric() {
                    number.push(char);
                    break;
                }
            }
            while let Some(char) = string.next() {
                if char.is_numeric() {
                    number.push(char);
                } else {
                    break;
                }
            }
            assert!(!number.is_empty(), "Could not find number");
            number.parse().unwrap()
        }
        Sensor {
            position: (find_number(&mut string), find_number(&mut string)),
            beacon: (find_number(&mut string), find_number(&mut string)),
        }
    }
}

fn distance(sensor: &Sensor) -> i64 {
    (sensor.position.0.abs_diff(sensor.beacon.0) + sensor.position.1.abs_diff(sensor.beacon.1))
        as i64
}

fn spaces(row: &Vec<RangeInclusive<i64>>, beacons: &Option<&HashSet<i64>>) -> i64 {
    let mut result = 0;
    for range in row {
        result += range.end() - range.start() + 1;
        if let Some(beacons) = beacons {
            for beacon in *beacons {
                if beacon >= range.start() && beacon <= range.end() {
                    result -= 1;
                }
            }
        }
    }
    result
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let sensors: Vec<Sensor> = file.trim().split('\n').map(Sensor::from).collect();
    let mut map: HashMap<i64, Vec<RangeInclusive<i64>>> = HashMap::new();
    let mut beacons: HashMap<i64, HashSet<i64>> = HashMap::new();
    println!("Processing sensors");
    let mut i = 0;
    for sensor in &sensors {
        i += 1;
        println!("Sensor {} of {}", i, sensors.len());
        if let Some(existing) = beacons.get_mut(&sensor.beacon.1) {
            existing.insert(sensor.beacon.0);
        } else {
            beacons.insert(sensor.beacon.1, HashSet::from([sensor.beacon.0]));
        }
        let distance = distance(sensor);
        for y in sensor.position.1 - distance..=sensor.position.1 + distance {
            let distance = distance - (y.abs_diff(sensor.position.1)) as i64;
            let range = sensor.position.0 - distance..=sensor.position.0 + distance;
            if let Some(existing) = map.get_mut(&y) {
                existing.push(range);
            } else {
                map.insert(y, vec![range]);
            }
        }
    }

    println!("Finding maximums");
    let mut min_x = i64::MAX;
    let mut max_x = 0;
    for range in map.values().flatten() {
        max_x = max_x.max(*range.end());
        min_x = min_x.min(*range.start());
    }

    // Fuse the rows together.
    println!("Fusing rows together");
    let mut row_n = 0;
    let row_total = map.len();
    for row in map.values_mut() {
        row_n += 1;
        if row_n % 10_000 == 0 {
            println!("Row {} of {}", row_n, row_total);
        }
        let mut i = 0;
        while i < row.len() {
            let mut j = 0;
            while j < row.len() {
                if i == j {
                    j += 1;
                    continue;
                }
                if row[i].start() <= row[j].start() && row[i].end() >= row[j].start() {
                    // Overlapping on the left.
                    row[i] = *row[i].start()..=*row[i].end().max(row[j].end());
                    row.remove(j);
                    if i > j {
                        i -= 1;
                    }
                } else if row[i].start() <= row[j].end() && row[i].end() >= row[j].end() {
                    // Overlapping on the right.
                    row[i] = *row[i].start().min(row[j].start())..=*row[i].end();
                    row.remove(j);
                    if i > j {
                        i -= 1;
                    }
                } else if row[i].start() >= row[j].start() && row[i].end() <= row[j].end() {
                    // Inside.
                    row[i] = row[j].clone();
                    row.remove(j);
                    if i > j {
                        i -= 1;
                    }
                } else if row[i].start() <= row[j].start() && row[i].end() >= row[j].end() {
                    // Outside.
                    row.remove(j);
                    if i > j {
                        i -= 1;
                    }
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
        row.sort_unstable_by(|a, b| a.start().cmp(b.start()));
    }

    println!("Part 1");
    println!(
        "The result is {}",
        spaces(&map[&TARGET_Y], &beacons.get(&TARGET_Y))
    );

    println!("Part 2");
    for y in 0..=SEARCH_RANGE {
        let row = &map[&y];
        if row.len() != 2 {
            continue;
        }
        let first = &row[0];
        let second = &row[1];
        if second.start() - first.end() != 2 {
            continue;
        }
        if *first.start() >= 0 || *second.end() <= SEARCH_RANGE {
            continue;
        }
        let x = first.end() + 1;
        println!("A result is ({}, {})", x, y);
        println!("The tuning frequency is {}", x * 4_000_000 + y);
    }
    println!("Done.");
}
