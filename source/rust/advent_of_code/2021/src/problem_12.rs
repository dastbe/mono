use std::collections::HashMap;
use std::collections::HashSet;
use std::option::Option;


pub fn main() {
    let input = include_str!("problem_12.input");

    let mut edges = HashMap::new();
    let mut small_nodes = HashSet::new();
    for line in input.lines() {
        let mut nodes = line.split("-");
        let first = nodes.next().expect("first");
        let second = nodes.next().expect("second");

        edges.entry(first.to_string()).or_insert(Vec::new()).push(second.to_string());
        edges.entry(second.to_string()).or_insert(Vec::new()).push(first.to_string());

        if first.chars().all(|c| c.is_ascii_lowercase()) {
            small_nodes.insert(first.to_string());
        }

        if second.chars().all(|c| c.is_ascii_lowercase()) {
            small_nodes.insert(second.to_string());
        }
    }

    println!("edges={:?}", edges);
    println!("small_nodes={:?}", small_nodes);

    small_nodes.remove(&"start".to_string());
    let all_paths = search(&edges, "start".to_string(), &mut small_nodes);
    small_nodes.insert("start".to_string());

    println!("problem 12 part 1: {:?}", all_paths);

    small_nodes.remove(&"start".to_string());
    let all_paths_with_lucky_cave = search2(&edges, "start".to_string(), &mut small_nodes, false);
    small_nodes.insert("start".to_string());

    println!("problem 12 part 2: {:?}", all_paths_with_lucky_cave);
}

pub fn search(edges: &HashMap<String, Vec<String>>, curr_node: String, remaining_nodes: &mut HashSet<String>) -> u32 {
    // just stop here, as we can only visit end once
    if curr_node == "end".to_string() {
        return 1;
    }

    let mut paths = 0;

    for destination in &edges[&curr_node] {
        // process as lowercase constraint
        if remaining_nodes.contains(destination) {
            remaining_nodes.remove(destination);
            paths += search(edges, destination.clone(), remaining_nodes);
            remaining_nodes.insert(destination.clone());
        } 

        // process as repeatable constraint
        if destination.chars().all(|c| c.is_ascii_uppercase()) {
            paths += search(edges, destination.clone(), remaining_nodes);
        }
    }

    return paths;
}

pub fn search2(edges: &HashMap<String, Vec<String>>, curr_node: String, remaining_nodes: &mut HashSet<String>, used_lucky_cave: bool) -> u32 {
    let mut second_chance = HashSet::new();
    return search22(edges, curr_node, remaining_nodes, &mut second_chance, used_lucky_cave);
}

pub fn search22(edges: &HashMap<String, Vec<String>>, 
    curr_node: String, 
    remaining_nodes: &mut HashSet<String>, 
    second_chance: &mut HashSet<String>, 
    used_lucky_cave: bool) -> u32 {

    // just stop here, as we can only visit end once
    if curr_node == "end".to_string() {
        return 1;
    }

    let mut paths = 0;

    for destination in &edges[&curr_node] {
        // start condition
        if *destination == "start".to_string() {
            continue;
        }

        // process normally
        if remaining_nodes.contains(destination) {
            remaining_nodes.remove(destination);
            second_chance.insert(destination.clone());
            paths += search22(edges, destination.clone(), remaining_nodes, second_chance, used_lucky_cave);
            second_chance.remove(destination);
            remaining_nodes.insert(destination.clone());
        }

        // if we haven't taken a second chance and its in the second chance pool, take it
        if !used_lucky_cave && second_chance.contains(destination) {
            paths += search22(edges, destination.clone(), remaining_nodes, second_chance, true);
        }

        // process as repeatable constraint
        if destination.chars().all(|c| c.is_ascii_uppercase()) {
            paths += search22(edges, destination.clone(), remaining_nodes, second_chance, used_lucky_cave);
        }
    }

    return paths;
}
