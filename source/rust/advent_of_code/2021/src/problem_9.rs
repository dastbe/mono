use std::vec::Vec;
use std::collections::HashSet;
use std::collections::VecDeque;
use itertools::Itertools;


pub fn main() {
    let input = include_str!("problem_9.input");
    let heightmap: Vec<Vec<u32>> = input.lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).expect("is a digit"))
            .collect())
        .collect();

    let mut low_points = Vec::new();

    let mut risk_level = 0;
    for (i, row) in heightmap.iter().enumerate() {
        for (j, element) in row.iter().enumerate() {
            let mut is_min = true;

            if i > 0 {
                is_min &= heightmap[i-1][j] > *element;
            }

            if i < heightmap.len() - 1 {
                is_min &= heightmap[i+1][j] > *element;
            }

            if j > 0 {
                is_min &= heightmap[i][j-1] > *element;
            }

            if j < row.len() - 1 {
                is_min &= heightmap[i][j+1] > *element;
            }

            if is_min {
                risk_level += *element + 1;

                low_points.push((i as i32, j as i32));
            }
        }
    }
    println!("problem 9 part 1: {}", risk_level);

    let mut visited = HashSet::new();
    let mut areas = Vec::new();

    for (initial_i, initial_j) in low_points {
        if visited.contains(&(initial_i, initial_j)) {
            continue;
        }

        let mut next = VecDeque::new();
        next.push_back((initial_i, initial_j));

        let mut area = 0;
        while !next.is_empty() {
            let (i, j) = next.pop_front().expect("verified non-empty");

            if i < 0 || i >= heightmap.len() as i32 || j < 0 || j >= heightmap[0].len() as i32 {
                continue
            }

            if visited.contains(&(i, j)) {
                continue;
            }

            let curr = heightmap[i as usize][j as usize];
            if curr == 9 {
                continue;
            }

            visited.insert((i, j));
            area += 1;

            next.push_back((i-1, j));
            next.push_back((i+1, j));
            next.push_back((i, j-1));
            next.push_back((i, j+1));
        }

        areas.push(area);
    }

    println!("problem 9 part 2: {}", areas.iter().sorted().rev().take(3).product::<u32>());
}
