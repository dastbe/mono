use std::vec::Vec;
use itertools::Itertools;


pub fn main() {
    let input = include_str!("problem_6.input");

    // whenever we spawn, we increment this
    // needs to be a big boy for exponential growth
    let mut total_fish: u64 = 0;
    let mut spawn_on_day: Vec<u64> = vec![0; 300];

    for entry in input.trim().split(",") {
        let idx = entry.parse::<usize>().expect("number");
        spawn_on_day[idx+1] += 1;
        total_fish += 1;
    }

    for i in 0..=256 {
        let spawners = spawn_on_day[i];
        spawn_on_day[i+9] += spawners;
        spawn_on_day[i+7] += spawners;
        total_fish += spawners;

        if i == 80 {
            println!("problem 6 part 1: {}", total_fish);
        }
        if i == 256 {
            println!("problem 6 part 2: {}", total_fish);
        }

    }
}
