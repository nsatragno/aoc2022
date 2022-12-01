use std::{fs, ops::AddAssign};

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut elves: Vec<u32> = vec![0];
    for line in file.trim().split('\n') {
        let line = line.trim();
        if line.is_empty() {
            elves.push(0);
        } else {
            elves
                .last_mut()
                .unwrap()
                .add_assign(line.parse::<u32>().unwrap());
        }
    }
    println!("{}", elves.iter().max().unwrap());

    elves.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let mut iter = elves.iter();
    let solution =
        iter.next().unwrap() + iter.next().unwrap() + iter.next().unwrap();
    println!("{}", solution);
}
