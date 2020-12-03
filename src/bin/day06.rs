use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input: String = fs::read_to_string("src/bin/input06.txt").expect("Could not read file");

    let re: Regex = Regex::new("^(turn on|turn off|toggle) (\\d+),(\\d+) through (\\d+),(\\d+)$").unwrap();
    // for part 1, flip to boolean and reimplement toggle appropriately (val = ! val) and count of just trues
    let mut grid: HashMap<(usize, usize), usize> = HashMap::new();

    for line in input.lines() {
        // println!("evaluating line: {}", line);
        for capture in re.captures_iter(line) {
            let action: &str = &capture[1];
            let x1: usize = capture[2].parse::<usize>().expect("couldn't find a number");
            let y1: usize = capture[3].parse::<usize>().expect("couldn't find a number");
            let x2: usize = capture[4].parse::<usize>().expect("couldn't find a number");
            let y2: usize = capture[5].parse::<usize>().expect("couldn't find a number");

            if x1 > x2 || y1 > y2 {
                panic!("bad input");
            }

            // println!("    {} {},{} to {},{}", action, x1, y1, x2, y2);

            for x in x1..=x2 {
                for y in y1..=y2 {
                    match action {
                        "turn on" => grid.entry((x, y)).and_modify(|l| *l = *l + 1 ).or_insert(1),
                        "turn off" => {
                            grid.entry((x, y))
                                .and_modify(|l| {
                                    if *l > 0 {
                                        *l = *l - 1;
                                    }
                                })
                                .or_insert(0)

                        },
                        "toggle" => grid.entry((x, y)).and_modify(|l| *l = *l + 2 ).or_insert(2),
                        _ => panic!("unexpected action")
                    };
                }
            }
        }
    }

    let mut count: usize = 0;

    for value in grid.values() {
        count += *value
    }

    println!("{}", count);
}