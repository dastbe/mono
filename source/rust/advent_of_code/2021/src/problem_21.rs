pub fn main() {
    {
        let mut p1_pos = 2;
        let mut p2_pos = 0;
        let mut p1_score = 0;
        let mut p2_score = 0;

        let mut die = 0;
        let mut turn = 0;


        while p1_score < 1000 &&  p2_score < 1000 {
            let mut to_move = 0;
            for _ in 0..3 {
                to_move += die + 1;
                die += 1;
                die %= 100;
            }

            // player1
            if turn % 2 == 0 {
                p1_pos += to_move;
                p1_pos %= 10;

                p1_score += if p1_pos == 0 {
                    10
                } else {
                    p1_pos
                }
            } else { // player 2
                p2_pos += to_move;
                p2_pos %= 10;

                p2_score += if p2_pos == 0 {
                    10
                } else {
                    p2_pos
                }
            }

            turn += 1;
        }

        println!("problem 21 part 1: {}", std::cmp::min(p1_score, p2_score) * turn * 3);
    }

    println!("problem 21 part 2: {:?}", search(0, 2, 0, 0, 0));
}

pub fn search(turn: u64, p1_pos: u64, p2_pos: u64, p1_score: u64, p2_score: u64) -> (u64, u64, u64) {
    // we've won
    if p1_score >= 21 {
        return (1, 0, 1);
    } else if p2_score >= 21 {
        return (0, 1, 1);
    }

    let paths_to = [0,0,0,1,3,6,7,6,3,1];
    let mut p1_wins = 0;
    let mut p2_wins = 0;
    let mut calls = 1;

    // p1
    if turn % 2 == 0 {
        for i in 3..=9 {
            let multiplier = paths_to[i];
            let p1_next_pos = (p1_pos + i as u64) % 10;
            let p1_next_score = p1_score + if p1_next_pos == 0 {
                10
            } else {
                p1_next_pos
            };

            let (p1_next_wins, p2_next_wins, subcalls) = search(turn+1, p1_next_pos, p2_pos, p1_next_score, p2_score);
            p1_wins += multiplier * p1_next_wins;
            p2_wins += multiplier * p2_next_wins;
            calls +=  subcalls;
        }
    } else {
        for i in 3..=9 {
            let multiplier = paths_to[i];
            let p2_next_pos = (p2_pos + i as u64) % 10;
            let p2_next_score = p2_score + if p2_next_pos == 0 {
                10
            } else {
                p2_next_pos
            };

            let (p1_next_wins, p2_next_wins, subcalls) = search(turn+1, p1_pos, p2_next_pos, p1_score, p2_next_score);
            p1_wins += multiplier * p1_next_wins;
            p2_wins += multiplier * p2_next_wins;
            calls += subcalls
        }
    }

    return (p1_wins,p2_wins, calls);
}

// state = (turn % 2, p1_pos, p2_pos, p1_score, p2_score) -> (p1_wins, p2_wins)
// initial = (0, 2, 0, 0, 0)
// boundary conditions -> if p1_score >= 21 || p2_score >= 21 then (1,0) or (0,1)
// recursion, for 3-9, calculate next state and multiply by number of rolls that get to next state
// after 3 rolls, we have 3^3 = 27 possible increments
// 3 = 1
// 4 = 3
// 5 = 6
// 6 = 7
// 7 = 6
// 8 = 3
// 9 = 1
