use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let matrix: Vec<Vec<i8>> = file
        .trim()
        .split('\n')
        .map(|line| {
            line.trim()
                .chars()
                .map(|character| character.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect();

    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    // From the top:
    for x in 0..matrix.len() {
        let mut largest = -1;
        for y in 0..matrix.len() {
            if matrix[x][y] > largest {
                visible.insert((x, y));
                largest = matrix[x][y];
            }
        }
    }

    // From the bottom:
    for x in 0..matrix.len() {
        let mut largest = -1;
        for y in (0..matrix.len()).rev() {
            if matrix[x][y] > largest {
                visible.insert((x, y));
                largest = matrix[x][y];
            }
        }
    }

    // From the left:
    for y in 0..matrix.len() {
        let mut largest = -1;
        for x in 0..matrix.len() {
            if matrix[x][y] > largest {
                visible.insert((x, y));
                largest = matrix[x][y];
            }
        }
    }

    // From the right:
    for y in 0..matrix.len() {
        let mut largest = -1;
        for x in (0..matrix.len()).rev() {
            if matrix[x][y] > largest {
                visible.insert((x, y));
                largest = matrix[x][y];
            }
        }
    }

    println!("Visible: {}", visible.len());

    let mut tree_scores: HashMap<(usize, usize), u32> = HashMap::new();
    for a in 0..matrix.len() {
        for b in 0..matrix.len() {
            // Count up.
            let mut up = 0;
            for y in (0..b).rev() {
                up += 1;
                if matrix[a][y] >= matrix[a][b] {
                    break;
                }
            }
            // Count down.
            let mut down = 0;
            for y in (b + 1)..matrix.len() {
                down += 1;
                if matrix[a][y] >= matrix[a][b] {
                    break;
                }
            }
            // Count left.
            let mut left = 0;
            for x in (0..a).rev() {
                left += 1;
                if matrix[x][b] >= matrix[a][b] {
                    break;
                }
            }
            // Count right.
            let mut right = 0;
            for x in (a + 1)..matrix.len() {
                right += 1;
                if matrix[x][b] >= matrix[a][b] {
                    break;
                }
            }
            tree_scores.insert((a, b), up * down * left * right);
        }
    }

    let max = tree_scores.values().max().unwrap();
    println!("Max: {max}");
}
