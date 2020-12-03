use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input: String = fs::read_to_string("src/bin/input05.txt").expect("Could not read file");
    let mut counter: usize = 0;

    // println!("{}", is_nice_part2("qjhvhtzxzqqjkmpb"));

    // for part 1, swap to is_nice_part1(line)
    for line in input.lines() {
        if is_nice_part2(line) {
            counter += 1
        }
    }

    println!("{}", counter);
}

fn is_nice_part1(string: &str) -> bool {
    let mut vowel_counter: usize = 0;
    let mut double_found: bool = false;
    let bad_strings: Vec<&str> = vec!["ab", "cd", "pq", "xy"];
    let mut vowels: HashSet<char> = HashSet::new();

    for vowel in vec!['a', 'e', 'i', 'o', 'u'] {
        vowels.insert(vowel);
    }

    println!("for word: {}", string);

    for bad_string in bad_strings {
        let the_split: Vec<&str> = string.split(bad_string).collect();
        if the_split.len() > 1 {
            println!("  found bad string: {}", bad_string);
            return false;
        }
    }

    let mut prior: char = '\0';
    for c in string.chars() {

        if vowels.contains(&c) {
            vowel_counter += 1;
        }

        println!("  c: {} prior: {}", c, prior);

        if prior != '\0' && c == prior {
            double_found = true;
        }

        if double_found == true && vowel_counter >= 3 {
            return true;
        }

        prior = c;
    }

    false
}

fn is_nice_part2(string: &str) -> bool {
    let mut found_sandwich: bool = false;
    let mut found_dub: bool = false;
    let mut dub_indices: HashMap<String, Vec<usize>> = HashMap::new();

    println!("for string: {}", string);

    if string.len() < 4 {
        println!("  not long enough");
        return false;
    }

    let s1: &str = string.get(0..1).expect("bad string");
    let s2: &str = string.get(1..2).expect("bad string");
    let s3: &str = string.get(2..3).expect("bad string");

    let mut d1 = String::new();
    let mut d2 = String::new();
    d1.push_str(&s1);
    d1.push_str(&s2);
    d2.push_str(&s2);
    d2.push_str(&s3);
    dub_indices.insert(d1, vec!(0));
    dub_indices.insert(d2, vec!(1));

    for i in 3..string.len() {
        let cur: &str = string.get(i+0..i+1).expect("bad string");
        let p1:  &str = string.get(i-1..i+0).expect("bad string");
        let p2:  &str = string.get(i-2..i-1).expect("bad string");

        println!("  {:?} {:?} {:?}", p2, p1, cur);

        let mut dub = String::new();
        dub.push_str(&p1);
        dub.push_str(&cur);

        let vec = dub_indices.entry(dub).or_default();

        for index in (*vec).iter() {
            if *index != i-2 {
                found_dub = true;
                break;
            }
        }

        vec.push(i-1);

        found_sandwich = found_sandwich || p2 == cur;

        println!("    sandwich = {}, dub = {}", found_sandwich, found_dub);

        if found_sandwich == true && found_dub == true {
            return true;
        }
    }

    false
}