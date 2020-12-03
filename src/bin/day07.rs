use std::fs;
use regex::Regex;
use std::collections::HashMap;
use crate::Value::{Pattern, Literal};

enum Value<'a> {
    Pattern(&'a str),
    Literal(u16)
}

fn main() {
    let input: String = fs::read_to_string("src/bin/input07.txt").expect("Could not read file");
    let example: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    let mut setup: HashMap<&str, Value> = HashMap::new();

    let assign = Regex::new("^(.+) -> (\\w+)$").unwrap();
    let val_re = Regex::new("^([\\w\\d]+)$").unwrap();
    let not_re = Regex::new("^NOT ([\\w\\d]+)$").unwrap();
    let bin_re = Regex::new("^([\\w\\d]+) (AND|OR|LSHIFT|RSHIFT) ([\\w\\d]+)$").unwrap();

    for line in example.lines() {
        for captures in assign.captures_iter(line) {
            let left: &str = &captures[1];
            let key:  &str = &captures[2];

            println!("{} -> {}", left, key);

            setup.insert(key, Pattern(value));
        }
    }

    println!("{}", *get_value("a", &mut setup))
}

fn get_value(key: &str, values: &mut HashMap<&str, Value>) -> &u16 {
    let val_re = Regex::new("^(\\d+)$").unwrap();
    let var_re = Regex::new("^(\\w+)$").unwrap();
    let not_re = Regex::new("^NOT ([\\w\\d]+)$").unwrap();
    let bin_re = Regex::new("^([\\w\\d]+) (AND|OR|LSHIFT|RSHIFT) ([\\w\\d]+)$").unwrap();

    let v = values.get(key);

    if let Some(s) = v {
        return match s {
            Literal(l) => &l,
            Pattern(pattern) => {
                if val_re.is_match(pattern) {
                    for c in val_re.capture_iter(pattern) {
                        let lit: u16 = c[1].parse::<u16>().expect("could not parse literal");
                        values.insert(key, Literal(lit));
                        return &lit;
                    }
                }
                else if var_re.is_match(pattern) {
                    for c in var_re.capture_iter(pattern) {
                        let val: &u16 = get_value(&c[1], values);
                        values.insert(key, Literal(*val))
                    }
                }
            }
        }
    }

    panic!("could not return a valid value");
}