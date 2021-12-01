use itertools::Itertools;

pub fn main() {
    let depths = include!("problem_1.input");

    println!("problem 1 part 1 solution: {}", count_increases(&depths));
    println!("problem 1 part 2 solution: {}", count_smeared_increases(&depths));
}

pub fn count_increases(depths : &Vec<i32>) -> i32 {
    let mut increases = 0;

    for (prev, next) in depths.into_iter().tuple_windows() {
        if next > prev {
            increases += 1
        }
    }

    return increases
}

pub fn count_smeared_increases(depths : &Vec<i32>) -> i32 {
    let mut increases = 0;

    for (first, second, third, fourth) in depths.into_iter().tuple_windows() {
        let prev = first + second + third;
        let next = second + third + fourth;
        if next > prev {
            increases += 1
        }
    }

    return increases
}
