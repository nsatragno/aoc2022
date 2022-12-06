use std::fs;

struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

fn parse_instruction(line: &str) -> Instruction {
    let mut words = line.trim().split(' ');
    words.next(); // Skip the move.
    let count: usize = words.next().unwrap().parse().unwrap();
    words.next(); // Skip the from.
    let from: usize = words.next().unwrap().parse().unwrap();
    words.next(); // Skip the to.
    let to: usize = words.next().unwrap().parse().unwrap();
    Instruction { from, to, count }
}

fn result(stacks: Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .fold(String::new(), |mut string, stack| {
            string.push(*stack);
            string
        })
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    // Find the line the stack indices.
    let (divider_index, divider_line) = file
        .split('\n')
        .enumerate()
        .find(|line| line.1.starts_with(" 1"))
        .unwrap();

    // Find the largest number in said line, that's the number of stacks.
    let stack_count = divider_line
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| s.len() > 0)
        .map(|num| num.parse::<usize>().unwrap())
        .max()
        .unwrap();
    let mut parsed_stacks: Vec<Vec<char>> = Vec::new();
    parsed_stacks.resize(stack_count, Vec::new());

    // Stack the boxes.
    let lines: Vec<&str> = file.split('\n').take(divider_index).collect();
    for line in lines.iter().rev() {
        let mut iter = line.chars();
        // Eat the first [
        iter.next();

        let mut index = 0;
        while let Some(char) = iter.next() {
            if char != ' ' {
                parsed_stacks[index].push(char);
            }
            // Eat the ] [
            iter.next();
            iter.next();
            iter.next();
            index += 1;
        }
    }
    let parsed_stacks = parsed_stacks;
    let instructions: Vec<&str> = file.trim().split('\n').skip(divider_index + 2).collect();

    // Execute the instructions as part 1.
    let mut stacks = parsed_stacks.clone();
    for instruction in &instructions {
        let instruction = parse_instruction(instruction);
        for _ in 0..instruction.count {
            let target = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(target);
        }
    }

    println!("Part 1");
    println!("Result: {}", result(stacks));

    // Execute the instructions as part 2.
    let mut stacks = parsed_stacks.clone();
    for instruction in &instructions {
        let instruction = parse_instruction(instruction);
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..instruction.count {
            temp.push(stacks[instruction.from - 1].pop().unwrap());
        }
        for _ in 0..instruction.count {
            stacks[instruction.to - 1].push(temp.pop().unwrap());
        }
    }

    println!("Part 2");
    println!("Result: {}", result(stacks));
}
