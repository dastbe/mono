use std::vec::Vec;
use std::collections::HashMap;
use std::collections::HashSet;


pub fn main() {
    let close_map = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let input = include_str!("problem_10.input");
    let mut score = 0;
    let mut completion_scores = Vec::new();

    for line in input.lines() {
        let mut stack = Vec::new();
        let mut had_incorrect_close = false;

        for c in line.chars() {
            if HashSet::from(['(', '[', '{', '<']).contains(&c) {
                stack.push(c);
                continue;
            }

            if stack.is_empty() {
                // ignore for now?
                break;
            }

            let open = stack.pop().expect("non-empty stack");
            if open != close_map[&c] {
                score += match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0,
                };
                had_incorrect_close = true;
                break;
            }
        }

        if !stack.is_empty() && !had_incorrect_close {
            let mut completion_score: u64 = 0;
            while !stack.is_empty() {
                completion_score *= 5;

                let open = stack.pop().expect("non-empty");
                completion_score += match open {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                };
            }

            completion_scores.push(completion_score);
        }
    }
    completion_scores.sort();
    let completion_score_winner = completion_scores[completion_scores.len() / 2];

    println!("problem 10 part 1: {}", score);
    println!("problem 10 part 2: {:?}", completion_score_winner);
}
