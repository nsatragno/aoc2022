use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

type Coordinates = (usize, usize);

#[derive(Eq)]
struct Node {
    coordinates: Coordinates,
    expected_score: u32,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.expected_score.partial_cmp(&self.expected_score)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.expected_score.cmp(&self.expected_score)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.expected_score == other.expected_score
    }
}

fn find(target: u8, grid: &Vec<Vec<u8>>) -> Coordinates {
    let mut result = None;
    'done: for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == target {
                result = Some((i, j));
                break 'done;
            }
        }
    }
    result.expect(
        (String::from("Could not find target node ") + target.to_string().as_str()).as_str(),
    )
}

fn expected_distance(start: &Coordinates, end: &Coordinates) -> u32 {
    (start.0.abs_diff(end.0) + start.1.abs_diff(end.1)) as u32
}

fn make_path(
    predecessors: &HashMap<Coordinates, Coordinates>,
    mut current: Coordinates,
) -> Vec<Coordinates> {
    let mut path = vec![current];
    while let Some(next) = predecessors.get(&current) {
        path.push(*next);
        current = *next;
    }
    path.reverse();
    path
}

fn is_valid(current: Coordinates, next: Coordinates, grid: &Vec<Vec<u8>>) -> bool {
    let current = grid[current.0][current.1];
    let next = grid[next.0][next.1];
    next <= current + 1
}

fn shortest_path(
    grid: &Vec<Vec<u8>>,
    start: Coordinates,
    end: Coordinates,
) -> Option<Vec<Coordinates>> {
    let mut fringe: BinaryHeap<Node> = BinaryHeap::new();
    fringe.push(Node {
        coordinates: start,
        expected_score: 0,
    });

    let mut predecessors: HashMap<Coordinates, Coordinates> = HashMap::new();

    let mut actual_scores: HashMap<Coordinates, u32> = HashMap::new();
    actual_scores.insert(start, 0);

    let mut expected_scores: HashMap<Coordinates, u32> = HashMap::new();
    expected_scores.insert(start, expected_distance(&start, &end));

    while !fringe.is_empty() {
        let current = fringe.pop().unwrap();
        if current.coordinates == end {
            return Some(make_path(&predecessors, current.coordinates));
        }

        let mut neighbours = Vec::new();
        if current.coordinates.0 > 0 {
            let up = (current.coordinates.0 - 1, current.coordinates.1);
            if is_valid(current.coordinates, up, grid) {
                neighbours.push(up);
            }
        }
        if current.coordinates.0 < grid.len() - 1 {
            let down = (current.coordinates.0 + 1, current.coordinates.1);
            if is_valid(current.coordinates, down, grid) {
                neighbours.push(down);
            }
        }
        if current.coordinates.1 > 0 {
            let left = (current.coordinates.0, current.coordinates.1 - 1);
            if is_valid(current.coordinates, left, grid) {
                neighbours.push(left);
            }
        }
        if current.coordinates.1 < grid[0].len() - 1 {
            let right = (current.coordinates.0, current.coordinates.1 + 1);
            if is_valid(current.coordinates, right, grid) {
                neighbours.push(right);
            }
        }

        for neighbour in neighbours {
            let maybe_score = actual_scores[&current.coordinates] + 1;
            if maybe_score < *actual_scores.get(&neighbour).unwrap_or(&u32::MAX) {
                predecessors.insert(neighbour.clone(), current.coordinates);
                let expected_score = maybe_score + expected_distance(&neighbour, &end);
                expected_scores.insert(neighbour.clone(), expected_score.clone());
                actual_scores.insert(neighbour.clone(), maybe_score);
                if fringe
                    .iter()
                    .find(|node| node.coordinates == neighbour)
                    .is_none()
                {
                    fringe.push(Node {
                        coordinates: neighbour,
                        expected_score,
                    });
                }
            }
        }
    }

    None
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut grid: Vec<Vec<u8>> = file
        .trim()
        .split('\n')
        .map(|line| {
            line.trim()
                .as_bytes()
                .iter()
                .map(|num| *num)
                .collect::<Vec<u8>>()
        })
        .collect();

    let start = find(b'S', &grid);
    grid[start.0][start.1] = b'a';
    let end = find(b'E', &grid);
    grid[end.0][end.1] = b'z';

    let path = shortest_path(&grid, start, end);
    let path = path.expect("Could not find a path between start and end");
    println!("First part");
    println!("The result is {}", path.len() - 1);

    let mut starts = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'a' {
                starts.push((i, j));
            }
        }
    }

    let shortest_path = starts
        .iter()
        .map(|start| match shortest_path(&grid, *start, end) {
            Some(path) => path.len(),
            _ => usize::MAX,
        })
        .min()
        .unwrap();
    println!("Second part");
    println!("The result is {}", shortest_path - 1);
}
