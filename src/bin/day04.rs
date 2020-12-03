use std::fs;
use md5;

fn main() {
    let input: String = fs::read_to_string("src/bin/input04.txt").expect("Could not read file");
    let mut counter: usize = 0;

    loop {
        let token: String = format!("{}{}", input, counter);
        let hash: String = format!("{:x?}", md5::compute(token));
        let important_part: &str = &hash[..6];

        if important_part == "000000" {
            break;
        }

        if counter > 1_000_000_000 {
            panic!();
        }

        counter += 1;
    }

    println!("{}", counter)
}
