use std::collections::HashMap;
use itertools::Itertools;


pub fn main() {
    let input = include_str!("problem_14.input");
    let chain = "NBOKHVHOSVKSSBSVVBCS".to_string();

    let mut mappings = HashMap::new();
    for mapping in input.lines() {
        let mut parts = mapping.split(" -> ");
        let from = parts.next().expect("first");
        let mut from_parts = from.chars();
        let from_1 = from_parts.next().expect("char");
        let from_2 = from_parts.next().expect("char");
        let to = parts.next().expect("second").chars().next().expect("single char");

        mappings.insert((from_1, from_2), to);
    }

    let mut current_chain: HashMap<(char, char), i64> = HashMap::new();
    for (first, second) in chain.chars().tuple_windows() {
        let count = current_chain.entry((first, second)).or_insert(0);
        *count += 1;
    }

    for i in 0..60 {
        println!("{}", i);
        let mut next_chain = HashMap::new();
        for ((first, second), count) in current_chain {
            if mappings.contains_key(&(first, second)) {
                let insertion = mappings.get(&(first, second)).expect("mapping");
                *next_chain.entry((first, *insertion)).or_insert(0) += count;
                *next_chain.entry((*insertion, second)).or_insert(0) += count;
            } else {
                *next_chain.entry((first, second)).or_insert(0) += count;
            }
        }

        current_chain = next_chain;
        println!("{:?}", current_chain);
    }

    let mut counts = HashMap::new();
    counts.insert('S', 1);
    for ((first, second), count) in current_chain {
        *counts.entry(first).or_insert(0) += count;
    }
    println!("{:?}", counts);

    let mut max = 0;
    let mut min = i64::MAX;
    for (_, v) in counts {
        max = std::cmp::max(max, v);
        min = std::cmp::min(min, v);
    }

    println!("problem 14: {:?} - {:?} = {:?}", max, min, max - min);
}
