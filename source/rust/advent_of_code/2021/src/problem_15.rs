use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Reverse;


pub fn main() {
    let input = include_str!("problem_15.input");

    let mut risk_levels: HashMap<(i32, i32), u32> = HashMap::new();

    let mut base_i_max = 0;
    let mut base_j_max = 0;
    for (i, line) in input.lines().enumerate() {
        base_i_max = std::cmp::max(base_i_max, i as i32);
        for (j, c) in line.chars().enumerate() {
            base_j_max = std::cmp::max(base_j_max, j as i32);
            risk_levels.insert((i as i32, j as i32), c.to_digit(10).expect("digit"));
        }
    }

    println!("start expand");
    // 1 2 3 4 5 6 7 8 9 1  2  3  4  5  6  7  8  9  1  ideal
    // 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 base sum
    // 0 1 2 3 4 5 6 7 8 9  10 11 12 13 14 15 16 17 18 subtract 1
    // 0 1 2 3 4 5 6 7 8 0  1  2  3  4  5  6  7  8  9  mod 9
    // 1 2 3 4 5 6 7 8 9 1  2  3  4  5  6  7  8  9  1  add 1
    // expand in all directions 5 times
    //
    let mut i_max = 0;
    let mut j_max = 0;
    let mut expanded_risk_levels = HashMap::new();
    for ((ii, jj), risk) in risk_levels {
        for i in 0..=4 {
            for j in 0..=4 {
                let next_ii = (i as i32 * (base_i_max + 1)) + ii;
                let next_jj = (j as i32 * (base_j_max + 1)) + jj;
                i_max = std::cmp::max(i_max, next_ii);
                j_max = std::cmp::max(j_max, next_jj);

                // calculate modified risk
                let next_risk = ((risk + i as u32 + j as u32 - 1) % 9) + 1;
                expanded_risk_levels.insert((next_ii, next_jj), next_risk);
            }
        }
    }
    risk_levels = expanded_risk_levels;

    let mut cost_to: HashMap<(i32, i32), u32> = HashMap::new();
    for (i, j) in risk_levels.keys() {
        cost_to.insert((*i, *j), u32::MAX);
    }
    // not that it matters, but cost to start is 0
    cost_to.insert((0, 0), 0);

    let mut next = BinaryHeap::new();
    next.push(Reverse(State {
        path_cost: 0,
        position: (0, 0)
    }));

    println!("start search");
    while !next.is_empty() {
        let s = next.pop().expect("next").0;
        let (i, j) = s.position;

        for iota in [-1, 1] {
            if risk_levels.contains_key(&(i+iota, j)) && *cost_to.get(&(i+iota, j)).expect("cost") == u32::MAX {
                let risk_level = risk_levels.get(&(i+iota, j)).expect("risk");
                let prev_cost = cost_to.get(&(i+iota, j)).expect("value");
                let next_cost = std::cmp::min(*prev_cost, s.path_cost + *risk_level);
                cost_to.insert((i+iota, j), next_cost);

                next.push(Reverse(State {
                    path_cost: next_cost,
                    position: (i+iota, j),
                }));
            }

            if risk_levels.contains_key(&(i, j+iota)) && *cost_to.get(&(i, j+iota)).expect("cost") == u32::MAX {
                let risk_level = risk_levels.get(&(i, j+iota)).expect("risk");
                let prev_cost = cost_to.get(&(i, j+iota)).expect("value");
                let next_cost = std::cmp::min(*prev_cost, s.path_cost + *risk_level);
                cost_to.insert((i, j+iota), next_cost);

                next.push(Reverse(State {
                    path_cost: next_cost,
                    position: (i, j+iota),
                }));
            }
        }
    }

    /*
    for i in 0..=i_max {
        for j in 0..=j_max {
            print!("{}", risk_levels.get(&(i as i32, j as i32)).expect("risk"));
        }
        println!("");
    }
    */

    println!("problem 15 part 2: {:?}", cost_to.get(&(i_max, j_max)).expect("present"));
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct State {
    path_cost: u32,
    position: (i32, i32),
}
