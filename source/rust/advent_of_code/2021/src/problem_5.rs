use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Reverse;


pub fn main() {
    let input = include_str!("problem_5.input");

    let mut counter: HashMap<(i32, i32), u32> = HashMap::new();

    for line in input.lines() {
        let mut points = line.split(" -> ");
        let mut first = points.next().expect("first").split(",");
        let x0 = first.next().expect("x0").parse::<i32>().expect("number");
        let y0 = first.next().expect("y0").parse::<i32>().expect("number");

        let mut second = points.next().expect("second").split(",");
        let x1 = second.next().expect("x1").parse::<i32>().expect("number");
        let y1 = second.next().expect("y1").parse::<i32>().expect("number");

        // straight line
        if x0 == x1 || y0 == y1{
            for x in std::cmp::min(x0, x1)..=std::cmp::max(x0, x1) {
                for y in std::cmp::min(y0, y1)..=std::cmp::max(y0, y1) {
                    *counter.entry((x, y)).or_insert(0) += 1;
                }
            }
        } else {
            // ensure left to right for simplicity
            if x0 < x1 {
                if y0 < y1 {
                    for idx in 0..=(x1-x0) {
                        *counter.entry((x0+idx, y0+idx)).or_insert(0) += 1;
                    }
                } else {
                    for idx in 0..=(x1-x0) {
                        *counter.entry((x0+idx, y0-idx)).or_insert(0) += 1;
                    }
                }
            } else {
                if y1 < y0 {
                    for idx in 0..=(x0-x1) {
                        *counter.entry((x1+idx, y1+idx)).or_insert(0) += 1;
                    }
                } else {
                    for idx in 0..=(x0-x1) {
                        *counter.entry((x1+idx, y1-idx)).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    counter.retain(|_, v| *v > 1);
    println!("problem 5: {}", counter.len());
}
