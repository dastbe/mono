pub fn main() {
    {
        let mut horizontal = 0;
        let mut vertical = 0;

        let directions = include_str!("problem_2.input").lines();
        for direction in directions {
            let mut components = direction.split_whitespace();
            let d = components.next().expect("has direction");
            let v = components.next().expect("has value").parse::<i32>().expect("always an integer");

            if d == "forward" {
                horizontal += v;
            } else if d == "up" {
                vertical -= v;
            } else {
                vertical += v;
            }
        }

        println!("problem 2 part 1 solution: {}", horizontal * vertical);
    }


    {
        let mut horizontal = 0;
        let mut vertical = 0;
        let mut aim = 0;

        let directions = include_str!("problem_2.input").lines();
        for direction in directions {
            let mut components = direction.split_whitespace();
            let d = components.next().expect("has direction");
            let v = components.next().expect("has value").parse::<i32>().expect("always an integer");

            if d == "forward" {
                horizontal += v;
                vertical += aim * v;
            } else if d == "up" {
                aim -= v;
            } else {
                aim += v;
            }
        }

        println!("problem 2 part 2 solution: {}", horizontal * vertical);
    }
}
