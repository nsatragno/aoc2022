use std::{fs, collections::{HashMap}, time::{SystemTime}};

const ITERATIONS_PART_1: usize = 24;
const ITERATIONS_PART_2: usize = 32;

#[derive(Debug)]
enum Mineral {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Mineral {
    fn values() -> [Mineral; 4] {
        [
            Mineral::Ore,
            Mineral::Clay,
            Mineral::Obsidian,
            Mineral::Geode,
        ]
    }
}

#[derive(Debug)]
struct Cost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

impl Cost {
    fn from(string: &str) -> Cost {
        let costs = string.split_once("costs ").unwrap().1.trim();
        let costs = costs.split(" and ");
        let mut ore = 0;
        let mut clay = 0;
        let mut obsidian = 0;
        for cost in costs {
            let mut parts = cost.split_whitespace();
            let cost = parts.next().unwrap().parse().unwrap();
            match parts.next().unwrap() {
                "ore" => ore = cost,
                "clay" => clay = cost,
                "obsidian" => obsidian = cost,
                _ => panic!("Unknown ore"),
            }
        }
        Cost {
            ore,
            clay,
            obsidian,
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    ore: Cost,
    clay: Cost,
    obsidian: Cost,
    geode: Cost,
}

#[derive(Clone)]
struct Resources {
    ore: usize,
    ore_robots: usize,

    clay: usize,
    clay_robots: usize,

    obsidian: usize,
    obsidian_robots: usize,

    geode: usize,
    geode_robots: usize,
}

impl Resources {
    fn new() -> Resources {
        Resources {
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geode: 0,
            geode_robots: 0,
        }
    }
    fn harvest(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }

    fn spend(&mut self, cost: &Cost) {
        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obsidian -= cost.obsidian;
    }

    fn build_robot(&mut self, target: &Mineral) {
        match target {
            Mineral::Ore => self.ore_robots += 1,
            Mineral::Clay => self.clay_robots += 1,
            Mineral::Obsidian => self.obsidian_robots += 1,
            Mineral::Geode => self.geode_robots += 1,
        }
    }
}

impl Blueprint {
    fn from(string: &str) -> Blueprint {
        let mut parts = string.split('.');
        Blueprint {
            ore: Cost::from(parts.next().unwrap()),
            clay: Cost::from(parts.next().unwrap()),
            obsidian: Cost::from(parts.next().unwrap()),
            geode: Cost::from(parts.next().unwrap()),
        }
    }

    fn calculate_geodes(&self, iterations: usize) -> usize {
        let mut best_scores = HashMap::new();
        Mineral::values()
            .iter()
            .map(|mineral| self.find_geodes(mineral, iterations, Resources::new(), &mut best_scores))
            .max()
            .unwrap()
    }

    fn find_geodes(&self, target: &Mineral, mut iterations: usize, mut resources: Resources, best_scores: &mut HashMap<usize, usize>) -> usize {
        let cost = match target {
            Mineral::Ore => &self.ore,
            Mineral::Clay => &self.clay,
            Mineral::Obsidian => &self.obsidian,
            Mineral::Geode => &self.geode,
        };
        if cost.clay > 0 && resources.clay_robots == 0 ||
           cost.obsidian > 0 && resources.obsidian_robots == 0 {
          // We cannot possibly build the target, return what we have.
          return iterations * resources.geode_robots + resources.geode;
        }
        // Gather the resources to build the target.
        while resources.ore < cost.ore || resources.clay < cost.clay || resources.obsidian < cost.obsidian {
            iterations -= 1;
            resources.harvest();

            if iterations == 0 {
                // No more iterations remaining.
                return resources.geode;
            }
        }

        // Spend the resources to build the target.
        resources.spend(&cost);

        // Building it takes one iteration.
        iterations -= 1;
        resources.harvest();

        if iterations == 0 {
            // No more iterations remaining.
            return resources.geode;
        }

        // The new robot is ready.
        resources.build_robot(&target);

        // Calculate the score for this iteration.
        let score = iterations * resources.geode_robots + resources.geode;
        for i in iterations..ITERATIONS_PART_2 {
            // If there is a better score for the current |iteration| or older, no point continuing here.
            if let Some(better_score) = best_scores.get(&i) {
                if score < *better_score {
                    return score;
                }
            }
        }
        best_scores.insert(iterations, score);

        let mut max = 0;
        for target in &Mineral::values() {
            let geodes = self.find_geodes(target, iterations, resources.clone(), best_scores);
            if geodes > max {
                max = geodes;
            }
        }
        max
    }
}

fn main() {
    let then = SystemTime::now();
    let file = fs::read_to_string("input.txt").unwrap();
    let blueprints: Vec<Blueprint> = file.trim().split('\n').map(Blueprint::from).collect();
    let result: usize = blueprints
        .iter()
        .enumerate()
        .map(|(index, blueprint)| {
            println!("Calculating blueprint {} of {}", index + 1, blueprints.len());
            (index + 1) * blueprint.calculate_geodes(ITERATIONS_PART_1)
        })
        .inspect(|score| println!("The score is {score}"))
        .sum();
    println!("The result for part 1 is {result}");

    let result: usize = blueprints
        .iter()
        .take(3)
        .enumerate()
        .map(|(index, blueprint)| {
            println!("Calculating blueprint {} of {}", index + 1, 3);
            blueprint.calculate_geodes(ITERATIONS_PART_2)
        })
        .inspect(|score| println!("The score is {score}"))
        .reduce(|a, b| a * b).unwrap();
    println!("The result for part 2 is {result}");

    println!("Took {:?}", then.elapsed());
}
