use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let result: u32 = file.trim().split('\n').map(|line| {
        let mut parts = line.trim().split(' ');
        let opponent = parts.next().unwrap();
        let ours = parts.next().unwrap();

        // A for Rock, B for Paper, and C for Scissors.
        let ours = match ours {
            "X" => "A",
            "Y" => "B",
            "Z" => "C",
            _ => panic!("Unexpected input {}", ours)
        };
        let multiplier = if ours == opponent {
            3
        } else if (ours == "A" && opponent == "C") ||
        (ours == "B" && opponent == "A") ||
        (ours == "C" && opponent == "B") {
            6
        } else {
            0
        };

        multiplier + match ours {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!("Unexpected input {}", ours)
        }
    }).sum();

    println!("First part:");
    println!("The result is: {result}");

    let result: u32 = file.trim().split('\n').map(|line| {
        let mut parts = line.trim().split(' ');
        let opponent = parts.next().unwrap();
        let ours = parts.next().unwrap();

        // X loss, Y tie, Z win.
        let ours = match ours {
            "X" => {  // loss
                match opponent {
                    "A" => "C",
                    "B" => "A",
                    "C" => "B",
                    _ => panic!(),
                }
            }
            "Y" => opponent,
            "Z" => 
                match opponent {
                    "A" => "B",
                    "B" => "C",
                    "C" => "A",
                    _ => panic!(),
                }
            _ => panic!("Unexpected input {}", ours)
        };
        let multiplier = if ours == opponent {
            3
        } else if (ours == "A" && opponent == "C") ||
        (ours == "B" && opponent == "A") ||
        (ours == "C" && opponent == "B") {
            6
        } else {
            0
        };

        multiplier + match ours {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => panic!("Unexpected input {}", ours)
        }
    }).sum();
    println!("Second part:");
    println!("The result is: {result}");
}