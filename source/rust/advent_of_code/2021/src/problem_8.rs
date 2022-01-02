use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use itertools::Itertools;

pub fn main() {
    let canonical_renderings = HashMap::from([
        ("abcefg", 0),
        ("cf", 1),
        ("acdeg", 2),
        ("acdfg", 3),
        ("bcdf", 4),
        ("abdfg", 5),
        ("abdefg", 6),
        ("acf", 7),
        ("abcdefg", 8),
        ("abcdfg", 9),
    ]);

    let mut unique_digits_count = 0;
    let mut digit_sum = 0;
    {
        let lines = include_str!("problem_8.input").lines();

        for line in lines {
            let mut canonical_to_mapping = HashMap::new();
            let mut mapping_to_canonical = HashMap::new();

            let mut components = line.split(" | ");
            let digits = components.next().expect("has digits");

            let mut segment_counts = HashMap::new();
            for digit in digits.split_whitespace() {
                for segment in digit.chars() {
                    let count = segment_counts.entry(segment).or_insert(0);
                    *count += 1;
                }
            }

            // step 1: extract unique segments by count (b, e, f)
            for (segment, count) in segment_counts {
                match count {
                    4 => { canonical_to_mapping.insert('e', segment); mapping_to_canonical.insert(segment, 'e'); }
                    9 => { canonical_to_mapping.insert('f', segment); mapping_to_canonical.insert(segment, 'f'); }
                    6 => { canonical_to_mapping.insert('b', segment); mapping_to_canonical.insert(segment, 'b'); }
                    _ => { },
                }
            }

            // step 1.5: find the one, the not f is c
            for digit in digits.split_whitespace() {
                if digit.len() == 2 {
                    let mut ones = digit_to_set(digit);

                    // remove the mapped values
                    for (_, mapping) in canonical_to_mapping.clone() {
                        ones.remove(&mapping);
                    }

                    for remaining in ones {
                        canonical_to_mapping.insert('c', remaining);
                        mapping_to_canonical.insert(remaining, 'c');
                    }
                }
            }

            // step 2: find the one and find the seven, diff for a?
            let mut ones = HashSet::new();
            let mut sevens = HashSet::new();
            let mut fours = HashSet::new();
            for digit in digits.split_whitespace() {
                if digit.len() == 2 {
                    ones = digit_to_set(digit);
                }

                if digit.len() == 3 {
                    sevens = digit_to_set(digit);
                }

                if digit.len() == 4 {
                    fours = digit_to_set(digit);
                }
            }
            
            for remaining in sevens.difference(&ones) {
                canonical_to_mapping.insert('a', *remaining);
                mapping_to_canonical.insert(*remaining, 'a');
            }


            // step 3: find the one and the four, subtract b, find d?
            for remaining in fours.difference(&ones) {
                if !mapping_to_canonical.contains_key(&remaining) {
                    canonical_to_mapping.insert('d', *remaining);
                    mapping_to_canonical.insert(*remaining, 'd');
                }
            }

            // step 4: the remaining one is g
            for possible_last_segment in ['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
                if !mapping_to_canonical.contains_key(&possible_last_segment) {
                    canonical_to_mapping.insert('g', possible_last_segment);
                    mapping_to_canonical.insert(possible_last_segment, 'g');
                }
            }


            let mut evaluated_digit = 0;
            let outputs = components.next().expect("has outputs");
            for output in outputs.split_whitespace() {
                let x: String = output.chars().map(|c| mapping_to_canonical.get(&c).expect("all mappings present")).sorted().collect();
                let value = *canonical_renderings.get(x.as_str()).expect("is a canonical rendering");

                evaluated_digit *= 10;
                evaluated_digit += value;

                if value == 1 || value == 4 || value == 7 || value == 8 {
                    unique_digits_count += 1;
                }
            }

            digit_sum += evaluated_digit;
        }

        println!("problem 8 part 1: {}", unique_digits_count);
        println!("problem 8 part 2: {}", digit_sum);
    }
}

pub fn digit_to_set(s: &str) -> HashSet<char> {
    return HashSet::from_iter(s.chars());
}

// thought: we can rep segment mappings as seen -> canonical
// once we have canonicalized, we can sort and then compare it to the canonical string?


// a X
// b X
// c X
// d X
// e X
// f X
// g
