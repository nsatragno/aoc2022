use std::{collections::HashMap, fs};

const TOTAL_DISK: u64 = 70000000;
const MIN_FREE: u64 = 30000000;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut current: Vec<&str> = vec![""];
    let mut folders: HashMap<String, u64> = HashMap::new();
    folders.insert(String::from(""), 0);
    for line in file.trim().split('\n') {
        let line = line.trim();
        // Parse commands.
        if line == "$ cd /" {
            current = vec![""];
            continue;
        }
        if line.starts_with("$ cd ..") {
            current.pop();
            continue;
        }
        if line.starts_with("$ cd") {
            current.push(&line[5..]);
            let folder = current.join("/");
            if !folders.contains_key(&folder) {
                folders.insert(folder, 0);
            }
            continue;
        }
        // Ignore $ ls and dir.
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }
        // Parse files.
        for index in 0..current.len() {
            let folder = current[0..index + 1].join("/");
            let size: u64 = line.split(' ').next().unwrap().parse().unwrap();
            let current_size = folders.get(&folder).unwrap();
            folders.insert(folder, size + current_size);
        }
    }

    let size: u64 = folders
        .iter()
        .filter(|(_, size)| **size <= 100000)
        .map(|(_, size)| size)
        .sum();

    println!("Part 1");
    println!("Combined size of small dirs: {}", size);

    let total_size = folders.get("").unwrap();
    let free_space = TOTAL_DISK - total_size;
    let size_to_free = MIN_FREE - free_space;
    let folder_to_free: &u64 = folders
        .iter()
        .filter(|(_, size)| **size >= size_to_free)
        .map(|(_, size)| size)
        .min()
        .unwrap();

    println!("Part 2");
    println!("Size of min folder to delete: {}", folder_to_free);
}
