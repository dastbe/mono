use std::vec::Vec;
use std::collections::VecDeque;
use std::collections::HashSet;


pub fn main() {
    let input = include_str!("problem_11.input");

    let layout: Vec<Vec<u32>> = input.lines()
        .map(|l| l.chars()
            .map(|c| c.to_digit(10).expect("0-9"))
            .collect())
        .collect();

    let mut layout_for_part_1 = layout.clone();
    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += iterate(&mut layout_for_part_1);
    }
    println!("problem 11 part 1: {:?}", total_flashes);

    let mut layout_for_part_2 = layout.clone();
    let mut step = 0;
    loop {
        step += 1;
        let total_flashes = iterate(&mut layout_for_part_2);
        if total_flashes == 100 {
            break;
        }
    }

    println!("problem 11 part 2: {:?}", step);
}

pub fn iterate(layout: &mut Vec<Vec<u32>>) -> u32 {
    let mut total_flashes = 0;

    let rows = layout.len();
    let cols = layout[0].len();

    // track flashes for iteration
    let mut flashes = VecDeque::new();

    // get initial flashes
    for i in 0..rows {
        for j in 0..cols {
            layout[i][j] += 1;
            if layout[i][j] > 9 {
                flashes.push_back((i as i32, j as i32));
            }
        }
    }

    let mut flashed = HashSet::new();
    while !flashes.is_empty() {
        let (i, j) = flashes.pop_front().expect("has flash");

        // invalid coordinate
        if i < 0 || i >= rows as i32 || j < 0 || j >= cols as i32 {
            continue;
        }

        // has already flashed
        if flashed.contains(&(i, j)) {
            continue;
        }

        // valid and unflashed so we can increment it
        layout[i as usize][j as usize] += 1;

        // won't flash atm, so no further work
        if layout[i as usize][j as usize] <= 9 {
            continue;
        }

        // flashed so mark it
        total_flashes += 1;
        flashed.insert((i, j));

        for i_lambda in [-1,0,1] {
            for j_lambda in [-1,0,1] {
                flashes.push_back((i+i_lambda,j+j_lambda));
            }
        }
    }

    // force all flashes to 0
    for (i, j) in flashed {
        layout[i as usize][j as usize] = 0;
    }

    return total_flashes;
}

pub fn dump_layout(layout: &Vec<Vec<u32>>) {
    for row in layout {
        println!("{:?}", row);
    }
    println!("----------");
}

// problem 11 part 1: 1571
// problem 11 part 2: 387
