use std::{collections::HashMap, fs};

fn dijkstra(source: &str, cave: &HashMap<String, Vec<String>>) -> HashMap<String, u32> {
    let mut distance: HashMap<String, u32> = HashMap::new();
    let mut queue = HashMap::new();
    for node in cave.keys() {
        if node == source {
            queue.insert(node.clone(), 0);
            distance.insert(node.to_string(), 0);
            continue;
        }
        distance.insert(node.clone(), u32::MAX - 1);
        queue.insert(node.clone(), u32::MAX - 1);
    }

    while !queue.is_empty() {
        let current = queue.iter().min_by(|(_, a), (_, b)| a.cmp(b)).unwrap();
        let current = current.0.clone();
        queue.remove(&current);

        for neighbour in &cave[&current] {
            if queue.get(neighbour).is_none() {
                continue;
            }
            let maybe_distance = distance[&current] + 1;
            if maybe_distance < distance[neighbour] {
                distance.insert(neighbour.clone(), maybe_distance);
                queue.insert(neighbour.clone(), maybe_distance);
            }
        }
    }

    distance
}

fn find_max_flow(
    source: &str,
    distances: &HashMap<String, HashMap<String, u32>>,
    flows: &HashMap<String, u32>,
    time_left: u32,
) -> u32 {
    let flow = flows[source] * time_left;
    let mut flows = flows.clone();
    flows.remove(source);
    let mut max_flow = 0;
    for valve in flows.keys() {
        let distance = distances[source][valve];
        if (distance + 1) > time_left {
            continue;
        }
        let new_time_left = time_left - distance - 1;
        let new_flow = find_max_flow(valve, distances, &flows, new_time_left);
        if new_flow > max_flow {
            max_flow = new_flow;
        }
    }
    flow + max_flow
}

fn find_max_flow_elephant(
    santa: &str,
    elephant: &str,
    distances: &HashMap<String, HashMap<String, u32>>,
    flows: &HashMap<String, u32>,
    time_left_santa: u32,
    time_left_elephant: u32,
) -> u32 {
    let flow = flows[santa] * time_left_santa + flows[elephant] * time_left_elephant;
    let mut flows = flows.clone();
    flows.remove(santa);
    flows.remove(elephant);
    let mut max_flow = 0;
    let mut moved_santa = false;
    for santa_valve in flows.keys() {
        let distance_santa = distances[santa][santa_valve];
        if (distance_santa + 1) > time_left_santa {
            continue;
        }
        moved_santa = true;
        let new_time_left_santa = time_left_santa - distance_santa - 1;
        let mut moved_elephant = false;
        for elephant_valve in flows.keys() {
            if elephant_valve == santa_valve {
                continue;
            }
            let distance_elephant = distances[elephant][elephant_valve];
            if (distance_elephant + 1) > time_left_elephant {
                continue;
            }
            moved_elephant = true;
            let new_time_left_elephant = time_left_elephant - distance_elephant - 1;
            let new_flow = find_max_flow_elephant(
                santa_valve,
                elephant_valve,
                distances,
                &flows,
                new_time_left_santa,
                new_time_left_elephant,
            );
            if new_flow > max_flow {
                max_flow = new_flow;
            }
        }
        if !moved_elephant {
            let new_flow = find_max_flow(santa_valve, distances, &flows, new_time_left_santa);
            if new_flow > max_flow {
                max_flow = new_flow;
            }
        }
    }
    if !moved_santa {
        for elephant_valve in flows.keys() {
            let distance = distances[elephant][elephant_valve];
            if (distance + 1) > time_left_elephant {
                continue;
            }
            let new_time_left_elephant = time_left_elephant - distance - 1;
            let new_flow = find_max_flow(elephant_valve, distances, &flows, new_time_left_elephant);
            if new_flow > max_flow {
                max_flow = new_flow;
            }
        }
    }
    flow + max_flow
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut cave: HashMap<String, Vec<String>> = HashMap::new();
    let mut flows: HashMap<String, u32> = HashMap::new();
    let mut distances: HashMap<String, HashMap<String, u32>> = HashMap::new();
    for line in file.trim().split('\n') {
        let line = line.trim();
        let parts = line.split_once("alve ").unwrap().1;
        let valve = parts.split_once(" ").unwrap().0.to_string();
        let flow = parts.split_once("rate=").unwrap().1;
        let flow = flow.split_once(";").unwrap().0.parse().unwrap();
        let edges = parts.split_once("to val").unwrap().1;
        let edges = edges.split_once(" ").unwrap().1;
        let edges: Vec<String> = edges.split(", ").map(|str| str.to_string()).collect();
        cave.insert(valve.clone(), edges);
        if flow != 0 {
            flows.insert(valve, flow);
        }
    }
    flows.insert(String::from("AA"), 0);

    for node in &cave {
        println!("{:?}", node);
    }
    for node in &flows {
        println!("{:?}", node);
    }

    for node in flows.keys() {
        distances.insert(node.clone(), dijkstra(node, &cave));
    }

    for node in &distances {
        println!("{:?}", node);
    }

    let result = find_max_flow("AA", &distances, &flows, 30);
    println!("The result is {result}");

    let result = find_max_flow_elephant("AA", "AA", &distances, &flows, 26, 26);
    println!("The result is {result}");
}
