use std::{collections::VecDeque, fs};

const ROUNDS_PART_1: u8 = 20;
const ROUNDS_PART_2: u64 = 10_000;

#[derive(Debug)]
enum Operation {
    Multiply,
    Add,
}

impl Operation {
    fn from(string: &str) -> Operation {
        match string {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("Unknown operation {}", string),
        }
    }

    fn operate(&self, item1: i64, item2: i64) -> i64 {
        match self {
            Operation::Add => item1 + item2,
            Operation::Multiply => item1 * item2,
        }
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Constant(i64),
}

impl Operand {
    fn from(string: &str) -> Operand {
        match string {
            "old" => Operand::Old,
            n => Operand::Constant(n.parse().unwrap()),
        }
    }

    fn to(&self, old_item: i64) -> i64 {
        match self {
            Operand::Old => old_item,
            Operand::Constant(constant) => *constant,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    operand1: Operand,
    operand2: Operand,
    divisible: i64,
    monkey_true: usize,
    monkey_false: usize,
    inspect_count: u64,
}

impl Monkey {
    fn from(string: &str) -> Monkey {
        let mut parts = string.split('\n');

        let items = &parts.nth(1).unwrap().trim()["Starting items: ".len()..];
        let items = items
            .split(", ")
            .map(str::parse)
            .map(Result::unwrap)
            .collect();
        let operation = &parts.next().unwrap().trim()["Operation: new = ".len()..];

        let mut expression = operation.split(' ');
        let operand1 = Operand::from(expression.next().unwrap());
        let operation = Operation::from(expression.next().unwrap());
        let operand2 = Operand::from(expression.next().unwrap());

        let divisible = &parts.next().unwrap().trim()["Test: divisible by ".len()..];
        let divisible = divisible.parse().unwrap();

        fn parse_monkey(string: &str) -> usize {
            string
                .trim()
                .split(' ')
                .map(|part| part.parse())
                .filter(Result::is_ok)
                .next()
                .unwrap()
                .unwrap()
        }

        let monkey_true = parse_monkey(parts.next().unwrap());
        let monkey_false = parse_monkey(parts.next().unwrap());
        Monkey {
            items,
            operation,
            operand1,
            operand2,
            divisible,
            monkey_true,
            monkey_false,
            inspect_count: 0,
        }
    }

    /// Returns a tuple containing the item thrown and the monkey it is thrown to.
    fn inspect_item(&mut self) -> Option<(i64, usize)> {
        let item = self.items.pop_front();
        if item.is_none() {
            return None;
        }
        self.inspect_count += 1;
        let item = item.unwrap();
        let item = self
            .operation
            .operate(self.operand1.to(item), self.operand2.to(item));
        let item = item / 3;
        let target_monkey = if item % self.divisible == 0 {
            self.monkey_true
        } else {
            self.monkey_false
        };
        Some((item, target_monkey))
    }

    /// Returns a tuple containing the item thrown and the monkey it is thrown to.
    fn inspect_item2(&mut self, limit: i64) -> Option<(i64, usize)> {
        let item = self.items.pop_front();
        if item.is_none() {
            return None;
        }
        self.inspect_count += 1;
        let item = item.unwrap();
        let item = self
            .operation
            .operate(self.operand1.to(item), self.operand2.to(item));
        let item = item % limit;
        let target_monkey = if item % self.divisible == 0 {
            self.monkey_true
        } else {
            self.monkey_false
        };
        Some((item, target_monkey))
    }
}

fn result(monkeys: &Vec<Monkey>) -> u64 {
    let mut inspections: Vec<u64> = monkeys.iter().map(|monkey| monkey.inspect_count).collect();
    inspections.sort_unstable();
    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut monkeys: Vec<Monkey> = file.trim().split("\r\n\r\n").map(Monkey::from).collect();

    for _ in 0..ROUNDS_PART_1 {
        for current_monkey in 0..monkeys.len() {
            while let Some((item, target_monkey_index)) = monkeys[current_monkey].inspect_item() {
                monkeys[target_monkey_index].items.push_back(item);
            }
        }
    }
    println!("The result is {}", result(&monkeys));

    let mut monkeys: Vec<Monkey> = file.trim().split("\r\n\r\n").map(Monkey::from).collect();
    let limit = monkeys
        .iter()
        .map(|monkey| monkey.divisible)
        .reduce(|a, b| a * b)
        .unwrap();
    for _ in 0..ROUNDS_PART_2 {
        for current_monkey in 0..monkeys.len() {
            while let Some((item, target_monkey_index)) =
                monkeys[current_monkey].inspect_item2(limit)
            {
                monkeys[target_monkey_index].items.push_back(item);
            }
        }
    }
    println!("The result is {}", result(&monkeys));
}
