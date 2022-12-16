use std::{collections::{HashMap, HashSet}, fs, ops::RangeInclusive, str::Chars};

type Coordinate = (i32, i32);

const TARGET_Y: i32 = 2_000_000;
//const TARGET_Y: i32 = 10;

struct Sensor {
    position: Coordinate,
    beacon: Coordinate,
}

impl Sensor {
    fn from(string: &str) -> Sensor {
        let mut string = string.chars();
        fn find_number(string: &mut Chars) -> i32 {
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

fn distance(sensor: &Sensor) -> i32 {
    (sensor.position.0.abs_diff(sensor.beacon.0) + sensor.position.1.abs_diff(sensor.beacon.1))
        as i32
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let sensors: Vec<Sensor> = file.trim().split('\n').map(Sensor::from).collect();
    let mut map: HashMap<i32, Vec<RangeInclusive<i32>>> = HashMap::new();
    let mut beacons: HashMap<i32, HashSet<i32>> = HashMap::new();
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
            let distance = distance - (y.abs_diff(sensor.position.1)) as i32;
            let range = sensor.position.0 - distance..=sensor.position.0 + distance;
            if let Some(existing) = map.get_mut(&y) {
                existing.push(range);
            } else {
                map.insert(y, vec![range]);
            }
        }
    }

    println!("Finding maximums");
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for range in map.values().flatten() {
        max_x = max_x.max(*range.end());
        min_x = min_x.min(*range.start());
    }
    for point in map.keys() {
        min_y = min_x.min(*point);
        max_y = max_x.max(*point);
    }

    println!("Min X: {}", min_x);
    println!("Max X: {}", max_x);
    println!("Min Y: {}", min_y);
    println!("Min Y: {}", max_y);

    // Fuse the rows together.
    println!("Fusing rows together");
    let mut row_n = 0;
    let row_total = map.len();
    for row in map.values_mut() {
        row_n += 1;
        if row_n % 10_000 == 0  {
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
    }

    println!("Finding result");
    let mut result = 0;
    let row = &map[&TARGET_Y];
    for range in row {
        result += range.end() - range.start() + 1;
        if let Some(beacons) = beacons.get(&TARGET_Y) {
            for beacon in beacons {
                if beacon >= range.start() && beacon <= range.end() {
                    result -= 1;
                }
            }
        }
    }

    println!("The result is {result}");
}
