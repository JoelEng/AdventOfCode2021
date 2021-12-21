use std::collections::HashMap;
use itertools::Itertools;

const INPUT_FILE: &str = "input_small_example.txt";

fn main() {
    part_one();
}

fn get_input() -> HashMap<String, Vec<String>> {
    let input = std::fs::read_to_string(INPUT_FILE)
        .expect("Failed to read file");

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    
    for line in input.lines() {
        let (a, b) = line.split('-').next_tuple().unwrap();
        let (a, b) = (a.to_string(), b.to_string());
        graph.entry(a.clone()).or_insert(vec![]).push(b.clone());
        graph.entry(b).or_insert(vec![]).push(a);
    };
    graph
}

fn part_one() {
    let graph = get_input();
    println!("{}", routes_through(&"start".to_string(), &vec![], &graph));
}

///Takes a vector of all visited small caves, so as to avoid them.
/// 
/// Returns the number of possible routes through this cave
fn routes_through(cave: &String, has_visited: &Vec<String>, graph: &HashMap<String, Vec<String>>) -> u32 {
    if cave == "end" {
        1
    } else {
        let mut nbr_of_routes = 0;
        let mut has_visited_appended = has_visited.clone();
        if *cave == cave.to_lowercase() {
            has_visited_appended.push(cave.clone());
        }
        for neighbour in graph.get(cave).unwrap() {
            if !has_visited.contains(neighbour) {
                nbr_of_routes += routes_through(neighbour, &has_visited_appended, graph);
            }
        }
        nbr_of_routes
    }
}