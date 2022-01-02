pub fn main() {
    // target area: x=206..250, y=-105..-57
    let mut sum = 0;
    let mut best_max = 0;
    for x_vel in 19..=250 {
        for y_vel in -105..=105 {
            let mut x = 0;
            let mut y = 0;
            let mut x_i = x_vel;
            let mut y_i = y_vel;
            let mut max_y = 0;
            
            while x <= 250 && y >= -105 {
                // update positions
                x += x_i;
                y += y_i;

                max_y = std::cmp::max(max_y, y);

                // update vector
                if x_i > 0 {
                    x_i -= 1;
                } 
                y_i -= 1;

                if x >= 206 && x <= 250 && y >= -105 && y <= -57 {
                    sum += 1;
                    best_max = std::cmp::max(best_max, max_y);
                    break;
                }
            }
        }
    }

    println!("problem 17 part 1: {}", best_max);
    println!("problem 17 part 2: {}", sum);
}
