use std::fs;
use std::cmp;

fn main() {
    let input: String = fs::read_to_string("src/bin/input02.txt").expect("Could not read file");

    let mut sq_ft_paper: usize = 0;
    let mut ft_ribbon: usize = 0;

    for line in input.lines() {
        let mut sides : Vec<usize> = Vec::new();

        for s in line.split("x") {
            sides.push(s.parse::<usize>().expect("couldn't find a number"));
        }

        let a1 = sides[0] * sides[1];
        let a2 = sides[0] * sides[2];
        let a3 = sides[1] * sides[2];
        let min_area = cmp::min(a1, cmp::min(a2, a3));

        let w1 = 2 * sides[0] + 2 * sides[1];
        let w2 = 2 * sides[0] + 2 * sides[2];
        let w3 = 2 * sides[1] + 2 * sides[2];
        let min_wrap = cmp::min(w1, cmp::min(w2, w3));

        sq_ft_paper += 2 * a1 + 2 * a2 + 2 * a3 + min_area;
        ft_ribbon += (min_wrap + sides[0] * sides[1] * sides[2])
    }

    println!("paper: {}", sq_ft_paper);
    println!("ribbon: {}", ft_ribbon);
}
