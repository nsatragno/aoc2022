use std::{fs, str::Chars};

type Coordinate = (i32, i32);

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

    let mut max_x = 0;
    let mut max_y = 0;

    let mut min_x: i32 = 0;
    let mut min_y: i32 = 0;
    for sensor in &sensors {
        let distance = distance(&sensor);
        let right_shift = sensor.position.0 + distance;
        if right_shift > max_x {
            max_x = right_shift
        }
        let bottom_shift = sensor.position.1 + distance;
        if bottom_shift > max_y {
            max_y = bottom_shift;
        }
        let left_shift = sensor.position.0 - distance;
        if left_shift < min_x {
            min_x = left_shift;
        }
        let top_shift = sensor.position.1 - distance;
        if top_shift < min_y {
            min_y = top_shift;
        }
    }

    let shift_x = min_x.abs() as i32;
    let shift_y = min_y.abs() as i32;
    let mut map = vec![vec!['.'; (max_y + 1 + shift_y) as usize]; (max_x + 1 + shift_x) as usize];
    println!("Shift x: {}", shift_x);
    println!("Shift y: {}", shift_y);
    println!("Map x: {}", map.len());
    println!("Map y: {}", map[0].len());

    for sensor in &sensors {
        let x = (sensor.position.0 + shift_x) as usize;
        let y = (sensor.position.1 + shift_y) as usize;
        map[x][y] = 'S';

        let x = (sensor.beacon.0 + shift_x) as usize;
        let y = (sensor.beacon.1 + shift_y) as usize;
        map[x][y] = 'B';

        let distance = (sensor.position.0.abs_diff(sensor.beacon.0)
            + sensor.position.1.abs_diff(sensor.beacon.1)) as i32;
        for x in sensor.position.0 - distance..=sensor.position.0 + distance {
            let distance = distance - (x.abs_diff(sensor.position.0)) as i32;
            for y in sensor.position.1 - distance..=sensor.position.1 + distance {
                let x = (x + shift_x) as usize;
                let y = (y + shift_y) as usize;
                if map[x][y] == '.' {
                    map[x][y] = '#';
                }
            }
        }
    }

    let mut count = 0;
    for x in 0..map.len() {
        let point = map[x][(10 + shift_y) as usize];
        if point == '#' || point == 'S' {
            count += 1;
        }
    }

    print!("  ");
    for x in 0..map.len() {
        if (x as i32 - shift_x as i32) % 5 == 0 {
            print!("{:2} ", x as i32 - shift_x as i32);
        } else {
            print!(" ");
        }
    }
    print!("\n");
    for y in 0..map[0].len() {
        print!("{:2} ", y as i32 - shift_y as i32);
        for x in 0..map.len() {
            print!("{}", map[x][y]);
        }
        print!("\n");
    }
    println!("The count is {count}");
}
