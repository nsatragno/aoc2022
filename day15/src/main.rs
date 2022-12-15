use std::{fs, str::Chars, collections::HashMap};

type Coordinate = (i32, i32);

const DEBUG: bool = false;
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
    let mut map: HashMap<Coordinate, char> = HashMap::new();


    let mut i = 0;
    for sensor in &sensors {
        i += 1;
        println!("Sensor {} / {}", i, sensors.len());
        map.insert(sensor.position, 'S');
        map.insert(sensor.beacon, 'B');

        let distance = distance(sensor);
        /*for x in sensor.position.0 - distance..=sensor.position.0 + distance {
            let distance = distance - (x.abs_diff(sensor.position.0)) as i32;
            for y in sensor.position.1 - distance..=sensor.position.1 + distance {
                if y == TARGET_Y {
                    if map.get(&(x, y)).is_none() {
                        map.insert((x, y), '#');
                    }
                }
            }
        }*/
        for y in sensor.position.1 - distance..=sensor.position.1 + distance {
            if y != TARGET_Y {
                continue;
            }
            let distance = distance - (y.abs_diff(sensor.position.1)) as i32;
            for x in sensor.position.0 - distance..=sensor.position.0 + distance {
                if map.get(&(x, y)).is_none() {
                    map.insert((x, y), '#');
                }
            }
        }
    }

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;
    for point in map.keys() {
        min_x = min_x.min(point.0);
        min_y = min_y.min(point.1);
        max_x = max_x.max(point.0);
        max_y = max_y.max(point.1);
    }
    let mut count = 0;
    for x in min_x..=max_x {
        let point = map.get(&(x, TARGET_Y));
        if let Some(point) = point {
            if *point == '#' || *point == 'S' {
                count += 1;
            }
        }
    }

    if DEBUG {
        print!("  ");
        for x in min_x..=max_x {
            if x % 5 == 0 {
                print!("{:2} ", x);
            } else {
                print!(" ");
            }
        }
        print!("\n");
        for y in min_y..=max_y {
            print!("{:2} ", y);
            for x in min_x..=max_x {
                if let Some(value) = map.get(&(x, y)) {
                    print!("{}", value);
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
    }

    println!("The count is {count}");
}
