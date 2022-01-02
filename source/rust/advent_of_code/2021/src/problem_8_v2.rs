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
            let mut components = line.split(" | ");
            let digits = components.next().expect("has digits");

            for permutation in ('a'..'h').permutations(7) {
                // test the digits
            }
        }
    }
}

pub fn digit_to_set(s: &str) -> HashSet<char> {
    return HashSet::from_iter(s.chars());
}

