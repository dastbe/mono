use std::vec::Vec;
use std::collections::HashSet;


pub fn main() {
    let input = include_str!("problem_13.input");

    let mut dots = HashSet::new();

    for line in input.lines() {
        let mut pos = line.split(",");
        dots.insert((pos.next().expect("first").parse::<i32>().expect("number"), 
                pos.next().expect("second").parse::<i32>().expect("number")));
    }

    /*
       fold along x=655
       fold along y=447
       fold along x=327
       fold along y=223
       fold along x=163
       fold along y=111
       fold along x=81
       fold along y=55
       fold along x=40
       fold along y=27
       fold along y=13
       fold along y=6
    */
    dots = fold_x(&dots, 655);
    dots = fold_y(&dots, 447);
    dots = fold_x(&dots, 327);
    dots = fold_y(&dots, 223);
    dots = fold_x(&dots, 163);
    dots = fold_y(&dots, 111);
    dots = fold_x(&dots, 81);
    dots = fold_y(&dots, 55);
    dots = fold_x(&dots, 40);
    dots = fold_y(&dots, 27);
    dots = fold_y(&dots, 13);
    dots = fold_y(&dots, 6);


    for y in 0..6 {
        for x in 0..40 {
            if dots.contains(&(x,y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

pub fn fold_x(dots: &HashSet<(i32, i32)>, along: i32) -> HashSet<(i32, i32)> {
    let mut next = HashSet::new();

    for (x, y) in dots {
        let y_next = *y;

        let x_next = if *x > along {
            along - (*x - along)
        } else {
            *x
        };

        next.insert((x_next, y_next));
    }

    return next;
}

pub fn fold_y(dots: &HashSet<(i32, i32)>, along: i32) -> HashSet<(i32, i32)> {
    let mut next = HashSet::new();

    for (x, y) in dots {
        let x_next = *x;

        let y_next = if *y > along {
            along - (*y - along)
        } else {
            *y
        };

        next.insert((x_next, y_next));
    }

    return next;
}
