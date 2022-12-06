use std::{collections::HashSet, fs};

fn priority(character: u8) -> u8 {
    if character.is_ascii_lowercase() {
        character - b'a' + 1
    } else {
        character - b'A' + 27
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let result: u32 = file
        .trim()
        .split('\n')
        .map(|line| {
            let chars = line.trim().as_bytes();
            let size = chars.len() / 2;
            let mut set: HashSet<u8> = HashSet::new();
            for i in 0..size {
                set.insert(chars[i]);
            }
            chars
                .iter()
                .skip(size)
                .find(|character| set.contains(character))
                .unwrap()
        })
        .map(|character| priority(*character) as u32)
        .sum();
    println!("First part");
    println!("Result: {result}");

    let mut iter = file.trim().split('\n').peekable();
    let mut result: u32 = 0;
    while iter.peek().is_some() {
        let character = **iter
            .by_ref()
            .take(3)
            .map(|string| {
                string
                    .trim()
                    .as_bytes()
                    .iter()
                    .fold(HashSet::new(), |mut set, character| {
                        set.insert(character);
                        set
                    })
            })
            .reduce(|a, b| HashSet::from_iter(a.intersection(&b).cloned()))
            .unwrap()
            .iter()
            .next()
            .unwrap();
        result += priority(character) as u32;
    }
    println!("Second part");
    println!("Result: {result}");
}
