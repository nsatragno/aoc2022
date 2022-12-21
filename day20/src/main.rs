use std::fs;

fn mix(numbers: &mut Vec<(usize, i64)>) {
    for i in 0..numbers.len() {
        let (index, &number) = numbers
            .iter()
            .enumerate()
            .find(|(_, (index, _))| *index == i)
            .unwrap();
        let mut new_index = index as i64 + number.1;
        new_index %= numbers.len() as i64 - 1;
        if new_index < 0 {
            new_index += numbers.len() as i64 - 1;
        }
        let new_index = new_index as usize;

        numbers.remove(index);
        numbers.insert(new_index, number);
    }
}

fn print_coordinates(numbers: &Vec<(usize, i64)>) {
    let (zero_index, _) = numbers
        .iter()
        .enumerate()
        .find(|(_, (_, number))| *number == 0)
        .unwrap();
    let result = numbers[(zero_index + 1000) % numbers.len()].1
        + numbers[(zero_index + 2000) % numbers.len()].1
        + numbers[(zero_index + 3000) % numbers.len()].1;
    println!("The result is {result}");
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut numbers: Vec<(usize, i64)> = file
        .trim()
        .split('\n')
        .map(|line| line.trim().parse().unwrap())
        .enumerate()
        .collect();

    let mut part1 = numbers.clone();
    mix(&mut part1);
    print_coordinates(&part1);

    for i in 0..numbers.len() {
        numbers[i].1 *= 811589153;
    }
    for _ in 0..10 {
        mix(&mut numbers);
    }
    print_coordinates(&numbers);
}
