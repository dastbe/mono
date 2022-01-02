use std::collections::HashSet; 

pub fn main() {
    let input = include_str!("problem_3.input");
    let mut counts = [0; 12];
    let mut total_input = 0;

    let mut numbers: HashSet<String> = HashSet::new();
    for number in input.lines() {
        numbers.insert(number.to_string());

        total_input += 1;

        for (idx, bit) in number.chars().enumerate() {
            match bit {
                '1' => counts[idx] += 1,
                _   => ()
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;

    for count in counts {
        gamma *= 2;
        epsilon *= 2;

        if count > total_input/2 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("problem 3 part 1 solution: {}", gamma * epsilon);

    let mut oxygen_remaining = numbers.clone();
    for i in 0..12 {
        if oxygen_remaining.len() == 1 {
            break;
        }

        let mut count_ones = 0;
        let mut count_zeroes = 0;
        for number in &oxygen_remaining {
            match number.chars().nth(i).expect("bit") {
                '1' => count_ones += 1,
                 _  => count_zeroes += 1,
            }
        }

        if count_ones >= count_zeroes {
            oxygen_remaining.retain(|v| v.chars().nth(i).expect("bit") == '1');
        } else {
            oxygen_remaining.retain(|v| v.chars().nth(i).expect("bit") == '0');
        }
    }

    let mut co2_remaining = numbers.clone();
    for i in 0..12 {
        if co2_remaining.len() == 1 {
            break;
        }

        let mut count_ones = 0;
        let mut count_zeroes = 0;
        for number in &co2_remaining {
            match number.chars().nth(i).expect("bit") {
                '1' => count_ones += 1,
                 _  => count_zeroes += 1,
            }
        }

        if count_ones >= count_zeroes {
            co2_remaining.retain(|v| v.chars().nth(i).expect("bit") == '0');
        } else {
            co2_remaining.retain(|v| v.chars().nth(i).expect("bit") == '1');
        }
    }

    for only_oxygen in &oxygen_remaining {
        for only_co2 in &co2_remaining {
            let oxygen = i32::from_str_radix(&only_oxygen, 2).expect("number");
            let co2 = i32::from_str_radix(&only_co2, 2).expect("number");

            println!("problem 5 part 2: {} * {} = {}", only_oxygen, only_co2, oxygen * co2);
        }
    }
}

