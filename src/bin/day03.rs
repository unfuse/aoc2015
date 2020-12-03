use std::fs;
use std::collections::HashSet;

fn main() {
    let input: String = fs::read_to_string("src/bin/input03.txt").expect("Could not read file");

    let mut santa_x: isize = 0;
    let mut santa_y: isize = 0;
    let mut robo_x: isize = 0;
    let mut robo_y: isize = 0;

    let mut visits = HashSet::new();
    visits.insert((0, 0));

    let mut track_santa = true;

    for char in input.clone().chars() {
        if track_santa {
            match char {
                '^' => santa_y += 1,
                'v' => santa_y -= 1,
                '<' => santa_x -= 1,
                '>' => santa_x += 1,
                _ => panic!("found weird character")
            }
            visits.insert((santa_x.clone(), santa_y.clone()));
        }
        else {
            match char {
                '^' => robo_y += 1,
                'v' => robo_y -= 1,
                '<' => robo_x -= 1,
                '>' => robo_x += 1,
                _ => panic!("found weird character")
            }
            visits.insert((robo_x.clone(), robo_y.clone()));
        }

        // For day 1, comment this line out
        track_santa = !track_santa;
    }

    println!("{}", visits.len())
}
