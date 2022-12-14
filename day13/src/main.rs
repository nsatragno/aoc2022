use std::fs;

#[derive(Debug)]
enum Entry {
    Number(u32),
    List(Vec<Entry>),
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

impl Eq for Entry {}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap() == std::cmp::Ordering::Equal
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let (Entry::Number(left), Entry::Number(right)) = (self, other) {
            return if left < right {
                Some(std::cmp::Ordering::Less)
            } else if left == right {
                Some(std::cmp::Ordering::Equal)
            } else {
                Some(std::cmp::Ordering::Greater)
            };
        }
        let left = self.normalize();
        let right = other.normalize();

        let mut left = left.iter();
        let mut right = right.iter();
        while let Some(left) = left.next() {
            if let Some(right) = right.next() {
                match left.partial_cmp(right) {
                    Some(std::cmp::Ordering::Equal) => continue,
                    any => return any,
                }
            }
            // Right side ran out of items first.
            return Some(std::cmp::Ordering::Greater);
        }
        if right.next().is_some() {
            // Left side ran out of items first.
            return Some(std::cmp::Ordering::Less);
        }
        // Both lists had the same number of items.
        return Some(std::cmp::Ordering::Equal);
    }
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
        .filter(|(_, packet_pair)| {
            packet_pair.0.partial_cmp(&packet_pair.1).unwrap() != std::cmp::Ordering::Greater
        })
        .fold(0, |sum, (index, _)| sum + index + 1);

    println!("Part 1");
    println!("The result is {}", result);

    let separators = vec![Entry::from("[[2]])"), Entry::from("[[6]]")];
    let mut entries: Vec<Entry> = file
        .trim()
        .split('\n')
        .filter(|line| !line.trim().is_empty())
        .map(Entry::from)
        .chain(separators.clone())
        .collect();
    entries.sort_unstable();
    let result = entries
        .iter()
        .enumerate()
        .filter(|(_, entry)| separators.contains(entry))
        .fold(1, |accumulator, (index, _)| accumulator * (index + 1));
    println!("Part 2");
    println!("The result is {}", result);
}
