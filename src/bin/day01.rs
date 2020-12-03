use std::fs;

fn main() {
    let input: String = fs::read_to_string("src/bin/input01.txt").expect("Could not read file");

    let mut basement_index: Option<usize> = None;
    let mut floor: isize = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unknown character"),
        }
        if floor == -1 && basement_index == None {
            basement_index = Some(i);
        }
    }

    println!("{}", floor);
    if let Some(i) = basement_index {
        println!("{}", i + 1);
    }
}
