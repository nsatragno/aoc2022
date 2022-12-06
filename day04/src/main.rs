use std::{fs, ops::Range};

fn to_pair(chars: &str) -> Range<u32> {
    let mut ends = chars.split('-');
    ends.next().unwrap().parse().unwrap()..ends.next().unwrap().parse().unwrap()
}

fn to_pairs(line: &str) -> (Range<u32>, Range<u32>) {
    let mut pairs = line.trim().split(',');
    (
        to_pair(pairs.next().unwrap()),
        to_pair(pairs.next().unwrap()),
    )
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let result = file
        .trim()
        .split('\n')
        .map(to_pairs)
        .filter(|(first, second)| {
            (first.start <= second.start && first.end >= second.end)
                || (first.start >= second.start && first.end <= second.end)
        })
        .count();

    println!("Part 1");
    println!("Result: {result}");

    let result = file
        .trim()
        .split('\n')
        .map(to_pairs)
        .filter(|(first, second)| {
            (first.start <= second.start && first.end >= second.start)
                || (first.start <= second.end && first.end >= second.end)
                || (first.start >= second.start && first.end <= second.end)
        })
        .count();
    println!("Part 2");
    println!("Result: {result}");
}
