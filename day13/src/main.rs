use std::fs;

#[derive(Debug)]
enum Entry {
    Number(u32),
    List(Vec<Entry>),
}

#[derive(PartialEq)]
enum SortResult {
    Sorted,
    NotSorted,
    Inconclusive,
}

impl Clone for Entry {
    fn clone(&self) -> Self {
        match self {
            Entry::List(list) => Entry::List(list.clone()),
            Entry::Number(number) => Entry::Number(number.clone()),
        }
    }
}

     

impl Entry {
    fn from(string: &str) -> Entry {
        let string = string.trim();
        if string.starts_with("[") {
            let mut string = string.chars().peekable();
            string.next(); // Eat the [.
            let mut parsed: Vec<Entry> = Vec::new();
            let mut current = String::new();
            let mut brackets = 0;
            while let Some(char) = string.next() {
                if char == ',' && brackets == 0 {
                    if current.is_empty() {
                        panic!("Unexpexted comma");
                    }
                    parsed.push(Entry::from(current.as_str()));
                    current.clear();
                    continue;
                }
                if char == ']' {
                    if brackets == 0 {
                        if !current.is_empty() {
                            parsed.push(Entry::from(current.as_str()));
                            current.clear();
                        }
                        return Entry::List(parsed);
                    } else {
                        brackets -= 1;
                    }
                }
                if char == '[' {
                    brackets += 1;
                }
                current.push(char);
            }
            panic!("Could not parse {:?}", string);
        } else {
            Entry::Number(string.parse().unwrap())
        }
    }

    fn normalize(&self) -> Vec<Entry> {
        match self {
            Entry::List(list) => list.clone(),
            Entry::Number(number) => vec![Entry::Number(*number)],
        }
    }
}
fn is_sorted(left: &Entry, right: &Entry) -> SortResult {
    if let (Entry::Number(left), Entry::Number(right)) = (left, right) {
        return if left < right {
            SortResult::Sorted
        } else if left == right {
            SortResult::Inconclusive
        } else {
            SortResult::NotSorted
        }
    }
    let left = left.normalize();
    let right = right.normalize();

    let mut left = left.iter();
    let mut right = right.iter();
    while let Some(left) = left.next() {
        if let Some(right) = right.next() {
            match is_sorted(left, right) {
                SortResult::Inconclusive => continue,
                any => return any,
            }
        } 
        // Right side ran out of items first.
        return SortResult::NotSorted;
    }
    if right.next().is_some() {
        // Left side ran out of items first.
        return SortResult::Sorted;
    }
    // Both lists had the same number of items.
    SortResult::Inconclusive
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let result = file
        .trim()
        .split("\n\n")
        .map(|packet_pair| {
            let mut packets = packet_pair.split('\n').map(Entry::from);
            (packets.next().unwrap(), packets.next().unwrap())
        })
        .enumerate()
        .filter(|(_, packet_pair)| is_sorted(&packet_pair.0, &packet_pair.1) == SortResult::Sorted)
        .fold(0, |sum, (index, _)| sum + index + 1);

    println!("The result is {}", result);
}
