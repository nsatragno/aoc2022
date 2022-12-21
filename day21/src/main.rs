use std::{collections::HashMap, fs};

#[derive(Clone, Debug, PartialEq)]
enum Operation {
    Add,
    Sub,
    Multiply,
    Divide,
    Equality,
}

impl Operation {
    fn from(string: &str) -> Operation {
        match string {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "*" => Operation::Multiply,
            "/" => Operation::Divide,
            _ => panic!("Unknown operation"),
        }
    }

    fn operate(&self, a: f64, b: f64) -> f64 {
        match &self {
            Operation::Add => a + b,
            Operation::Sub => a - b,
            Operation::Multiply => a * b,
            Operation::Divide => a / b,
            Operation::Equality => panic!("Tried to operate on an equality"),
        }
    }

    fn invert(&self) -> Operation {
        match &self {
            Operation::Add => Operation::Sub,
            Operation::Sub => Operation::Add,
            Operation::Multiply => Operation::Divide,
            Operation::Divide => Operation::Multiply,
            Operation::Equality => panic!("Tried to invert an equality"),
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Operation::Add => '+',
            Operation::Sub => '-',
            Operation::Multiply => '*',
            Operation::Divide => '/',
            Operation::Equality => '=',
        }
        .fmt(f)
    }
}

#[derive(Clone, Debug)]
enum Expression {
    Constant(f64),
    Expression(String, Operation, String),
}

impl Expression {
    fn from(string: &str) -> Expression {
        if let Ok(number) = string.parse() {
            Expression::Constant(number)
        } else {
            let mut parts = string.split_whitespace();
            Expression::Expression(
                parts.next().unwrap().to_string(),
                Operation::from(parts.next().unwrap()),
                parts.next().unwrap().to_string(),
            )
        }
    }

    fn invert(&self) -> Operation {
        match &self {
            Expression::Expression(_, operation, _) => operation.invert(),
            _ => panic!("Attempted to invert a constant"),
        }
    }

    fn operation(&self) -> Operation {
        match &self {
            Expression::Expression(_, operation, _) => operation.clone(),
            _ => panic!("Attempted to invert a constant"),
        }
    }
}

#[derive(Clone, Debug)]
struct Branch {
    expression: Expression,
    has_variable: bool,
}

impl Branch {
    fn from(expression: Expression) -> Branch {
        Branch {
            expression,
            has_variable: false,
        }
    }
}

fn solve(root: &str, expressions: &HashMap<String, Expression>) -> f64 {
    match &expressions[root] {
        Expression::Constant(number) => *number,
        Expression::Expression(a, operation, b) => {
            operation.operate(solve(&a, expressions), solve(&b, expressions))
        }
    }
}

fn find_x(root: &str, expressions: &mut HashMap<String, Branch>) {
    if root == "humn" {
        expressions.get_mut(root).unwrap().has_variable = true;
        return;
    }
    let expression = &expressions.get(root).unwrap().expression.clone();
    if let Expression::Expression(a, _, b) = expression {
        find_x(a, expressions);
        find_x(b, expressions);
        assert!(!(expressions[a].has_variable & expressions[b].has_variable));
        if expressions[a].has_variable || expressions[b].has_variable {
            expressions.get_mut(root).unwrap().has_variable = true;
        }
    }
}

fn clear_x(root: &str, expressions: &mut HashMap<String, Branch>) {
    assert!(expressions[root].has_variable);
    loop {
        let (root_left_tag, root_right_tag) =
            if let Expression::Expression(left, _, right) = &expressions[root].expression {
                (left, right)
            } else {
                panic!("Unexpected constant on root");
            };

        let mut root_left = &expressions[root_left_tag];
        let root_right = &expressions[root_right_tag];

        // Always have the side with the variable on the "left".
        if root_right.has_variable {
            root_left = root_right;
        }
        if let Expression::Constant(_) = root_left.expression {
            return;
        }

        let (left_left_tag, left_right_tag) = if let Expression::Expression(left, _, right) =
            &expressions[root_left_tag].expression
        {
            (left, right)
        } else {
            panic!("Unexpected constant on root");
        };

        let left_left = &expressions[left_left_tag];
        let left_right = &expressions[left_right_tag];
        let passed_tag = root_left_tag;
        let (passed, new_root) = if left_right.has_variable {
            let operation = root_left.expression.operation();
            let passed = if operation == Operation::Divide || operation == Operation::Sub {
                Expression::Expression(
                    left_left_tag.clone(),
                    operation.clone(),
                    root_right_tag.clone(),
                )
            } else {
                Expression::Expression(
                    root_right_tag.clone(),
                    operation.invert(),
                    left_left_tag.clone(),
                )
            };
            let new_root = Expression::Expression(
                left_right_tag.clone(),
                Operation::Equality,
                passed_tag.clone(),
            );
            (passed, new_root)
        } else {
            assert!(left_left.has_variable);
            let passed = Expression::Expression(
                root_right_tag.clone(),
                root_left.expression.invert(),
                left_right_tag.clone(),
            );
            let new_root = Expression::Expression(
                left_left_tag.clone(),
                Operation::Equality,
                passed_tag.clone(),
            );
            (passed, new_root)
        };
        expressions.insert(passed_tag.clone(), Branch::from(passed));
        expressions.insert(root.to_string(), Branch::from(new_root));
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let expressions: HashMap<String, Expression> = file
        .trim()
        .split('\n')
        .map(|line| {
            let parts = line.split_once(": ").unwrap();
            (parts.0.to_string(), Expression::from(parts.1))
        })
        .collect();

    // Part 1.
    let result = solve("root", &expressions);
    println!("The result is {result}");

    // Part 2.
    let mut expressions: HashMap<String, Branch> = expressions
        .iter()
        .map(|(key, value)| (key.clone(), Branch::from(value.clone())))
        .collect();
    if let Expression::Expression(a, _, b) = &expressions["root"].expression {
        expressions.insert(
            "root".to_string(),
            Branch::from(Expression::Expression(
                a.to_string(),
                Operation::Equality,
                b.to_string(),
            )),
        );
    }
    find_x("root", &mut expressions);
    clear_x("root", &mut expressions);

    let expressions: HashMap<String, Expression> = expressions
        .iter()
        .map(|(key, value)| (key.clone(), value.expression.clone()))
        .collect();
    if let Expression::Expression(left, _, right) = &expressions["root"] {
        let result = if left == "humn" {
            solve(right, &expressions)
        } else {
            assert!(right == "humn");
            solve(left, &expressions)
        };
        println!("The result is {result}");
    } else {
        panic!("Did not properly clear X");
    }
}
