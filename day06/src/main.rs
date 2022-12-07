use std::{fs, collections::HashSet};

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let bytes = file.trim().as_bytes();
    for i in 3..bytes.len() {
        let mut set: HashSet<u8> = HashSet::new();
        set.insert(bytes[i - 3]);
        set.insert(bytes[i - 2]);
        set.insert(bytes[i - 1]);
        set.insert(bytes[i - 0]);
        if set.len() >= 4 {
            println!("Found at index {}", i + 1);
            break;
        }
    }

    for i in 14..bytes.len() {
        let mut set: HashSet<u8> = HashSet::new();
        for j in 0..14 {
            set.insert(bytes[i - j]);
            if set.len() >= 14 {
                println!("Found at index {}", i + 1);
                return;
            }
        }
    }
}
