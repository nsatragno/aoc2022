use std::fs;

enum Instruction {
    AddX(i32),
    Noop,
}

impl Instruction {
    fn from(line: &str) -> Instruction {
        let line = line.trim();
        if line == "noop" {
            Instruction::Noop
        } else if line.starts_with("addx") {
            Instruction::AddX(line.split_whitespace().nth(1).unwrap().parse().unwrap())
        } else {
            panic!("Unexpected pattern {}", line)
        }
    }

    fn cycles(&self) -> u32 {
        match self {
            Self::AddX(_) => 2,
            Self::Noop => 1,
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let mut clock: i32 = 0;
    let mut register_x: i32 = 1;
    let mut strength = 0;
    let mut screen = vec![vec!['.'; 40]; 6];
    for line in file.trim().split('\n') {
        let instruction = Instruction::from(line);
        for _ in 0..instruction.cycles() {
            let currently_drawn_x = clock % 40;
            if (register_x - currently_drawn_x).abs() <= 1 {
                let currently_drawn_y = clock / 40;
                screen[currently_drawn_y as usize][currently_drawn_x as usize] = '#';
            }
            clock += 1;
            if (clock + 20) % 40 == 0 {
                strength += clock * register_x;
            }
        }
        match instruction {
            Instruction::Noop => (),
            Instruction::AddX(operand) => register_x += operand,
        }
    }

    println!("The strength is {strength}");
    println!("Total cycles: {clock}");

    for line in screen {
        println!("{}", line.iter().collect::<String>());
    }
}
