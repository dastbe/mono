use std::vec::Vec;
use itertools::Itertools;


pub fn main() {
    let input = include_str!("problem_7.input");
    let positions: Vec<i32> = input.split(",")
        .map(|num| num.parse::<i32>().expect("number"))
        .sorted()
        .collect();

    let med = positions[positions.len()/2];
    let mut best_for_part_1 = 0;
    for position in &positions {
        best_for_part_1 += (med - position).abs();
    }

    println!("problem 7 part 1: {:?}", best_for_part_1);

    let mean = positions.iter().sum::<i32>()/positions.len() as i32;
    let mut best_for_part_2 = 0;
    for position in &positions {
        let dist = (mean - position).abs();
        best_for_part_2 += (dist * (dist+1)) / 2;
    }

    println!("problem 7 part 2: {:?}", best_for_part_2);
}
